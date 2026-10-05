use std::collections::HashMap;
use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender};
use crossterm::event::KeyEvent;

use veloce_core::{AnyView, Element, Flex, NavCommand, View, ViewAdapter};

/// Commands a view can issue toward the router (push/pop/replace).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouterCommand {
    Push(String),
    Pop,
    Replace(String),
}

pub struct ActiveRoute {
    pub path: String,
    pub view: Box<dyn AnyView>,
    pub is_modal: bool,
}

type RouteFn = Arc<dyn Fn(Sender<NavCommand>) -> Box<dyn AnyView> + Send + Sync>;

/// Persistent chrome rendered around the active route.
pub struct LayoutWrapper {
    pub top: Option<Element>,
    pub bottom: Option<Element>,
}

impl LayoutWrapper {
    pub fn new() -> Self {
        Self {
            top: None,
            bottom: None,
        }
    }

    pub fn with_top(mut self, el: Element) -> Self {
        self.top = Some(el);
        self
    }

    pub fn with_bottom(mut self, el: Element) -> Self {
        self.bottom = Some(el);
        self
    }

    pub fn compose(&self, body: Element) -> Element {
        let mut col = Flex::column();
        if let Some(top) = &self.top {
            col = col.child(top.clone());
        }
        col = col.child(body);
        if let Some(bottom) = &self.bottom {
            col = col.child(bottom.clone());
        }
        col.into_element()
    }
}

impl Default for LayoutWrapper {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Router {
    routes: HashMap<String, RouteFn>,
    pub stack: Vec<ActiveRoute>,
    wrapper: Option<LayoutWrapper>,
    nav_tx: Sender<NavCommand>,
    nav_rx: Receiver<NavCommand>,
}

impl Router {
    pub fn new() -> Self {
        let (nav_tx, nav_rx) = crossbeam_channel::bounded(64);
        Self {
            routes: HashMap::new(),
            stack: Vec::new(),
            wrapper: None,
            nav_tx,
            nav_rx,
        }
    }

    pub fn nav_sender(&self) -> Sender<NavCommand> {
        self.nav_tx.clone()
    }

    pub fn route<V>(&mut self, path: &str, view: V) -> &mut Self
    where
        V: View + Clone,
    {
        self.routes.insert(
            path.to_string(),
            Arc::new(move |nav| Box::new(ViewAdapter::new(view.clone(), nav)) as Box<dyn AnyView>),
        );
        self
    }

    pub fn set_wrapper(&mut self, wrapper: LayoutWrapper) -> &mut Self {
        self.wrapper = Some(wrapper);
        self
    }

    /// Reset the stack with the given route as the unpoppable root.
    pub fn root(&mut self, path: &str) -> &mut Self {
        let ctor = self.routes.get(path).expect("unknown route").clone();
        self.stack.clear();
        self.stack.push(ActiveRoute {
            path: path.to_string(),
            view: ctor(self.nav_tx.clone()),
            is_modal: false,
        });
        self
    }

    /// Push a new route on the stack; its view starts unloaded and
    /// hydrates via `load()`.
    pub fn push(&mut self, path: &str) -> &mut Self {
        let ctor = self.routes.get(path).expect("unknown route").clone();
        self.stack.push(ActiveRoute {
            path: path.to_string(),
            view: ctor(self.nav_tx.clone()),
            is_modal: false,
        });
        self
    }

    pub fn push_modal(&mut self, path: &str) -> &mut Self {
        let ctor = self.routes.get(path).expect("unknown route").clone();
        self.stack.push(ActiveRoute {
            path: path.to_string(),
            view: ctor(self.nav_tx.clone()),
            is_modal: true,
        });
        self
    }

    /// Refuses to pop the root route; returns false in that case.
    pub fn pop(&mut self) -> bool {
        if self.stack.len() <= 1 {
            return false;
        }
        if let Some(mut popped) = self.stack.pop() {
            popped.view.abort_load();
        }
        true
    }

    pub fn replace(&mut self, path: &str) -> &mut Self {
        if let Some(mut top) = self.stack.pop() {
            top.view.abort_load();
        }
        let ctor = self.routes.get(path).expect("unknown route").clone();
        self.stack.push(ActiveRoute {
            path: path.to_string(),
            view: ctor(self.nav_tx.clone()),
            is_modal: false,
        });
        self
    }

    pub fn top(&self) -> Option<&ActiveRoute> {
        self.stack.last()
    }

    /// Keys route to the topmost modal when one is active, otherwise to the
    /// top route; a view consuming the event prevents any further routing
    /// (global focus manager runs only when nothing consumed the event).
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if let Some(modal_index) = self
            .stack
            .iter()
            .rposition(|r| r.is_modal)
            .filter(|i| *i == self.stack.len() - 1)
        {
            return self.stack[modal_index].view.handle_key(key);
        }
        match self.stack.last_mut() {
            Some(route) => route.view.handle_key(key),
            None => false,
        }
    }

    /// Apply pending navigation commands issued via `cx.navigate/back/replace`.
    pub fn drain_command_queue(&mut self) {
        while let Ok(cmd) = self.nav_rx.try_recv() {
            match cmd {
                NavCommand::Push(p) => {
                    self.push(&p);
                }
                NavCommand::Pop => {
                    self.pop();
                }
                NavCommand::Replace(p) => {
                    self.replace(&p);
                }
            }
        }
    }

    /// Hydration + action steps. Call once per frame.
    pub fn step(&mut self, handle: &tokio::runtime::Handle) {
        for route in self.stack.iter_mut() {
            route.view.enter(handle);
            route.view.poll();
        }
        // Deliver queued actions to their owning route only.
        for route in self.stack.iter_mut() {
            route.view.drain_actions(64);
        }
    }

    /// Composes the current visual state as an `Element` tree.
    pub fn element(&self) -> Element {
        let composed = self.compose_stack();
        match &self.wrapper {
            Some(w) => w.compose(composed),
            None => composed,
        }
    }

    fn compose_stack(&self) -> Element {
        let Some(top) = self.stack.last() else {
            return Element::Text(veloce_core::Text::new("empty router"));
        };
        if top.is_modal && self.stack.len() >= 2 {
            let below = self.stack[self.stack.len() - 2].view.view();
            return Element::Overlay {
                below: Box::new(below),
                overlay: Box::new(top.view.view()),
            };
        }
        top.view.view()
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
