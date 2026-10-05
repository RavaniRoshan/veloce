use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use crossterm::event::KeyEvent;

use crate::context::{channel, BoxedAction, Context, Dispatcher, NavCommand};
use crate::element::{Element, Text};

/// An application screen. Implemented by users; never used as a trait
/// object directly — use `AnyView` for dynamic dispatch.
#[async_trait::async_trait]
pub trait View: Send + Sync + 'static {
    type Action: Send + Sync + Clone + 'static;

    /// Async data dependency. Runs on the Tokio runtime on route entry.
    /// While pending, `fallback()` is rendered.
    async fn load(&mut self) -> Result<()> {
        Ok(())
    }

    /// Rendered while `load()` is pending. Default: loading text.
    fn fallback(&self) -> Element {
        Element::Text(Text::new("loading…"))
    }

    fn update(&mut self, action: Self::Action, cx: &mut Context<Self::Action>);

    fn view(&self) -> Element;

    /// Intercept raw keys before the global focus manager.
    fn handle_key(&mut self, _key: KeyEvent, _cx: &mut Context<Self::Action>) -> bool {
        false
    }
}

/// Type-erased view used by the router. Implemented by `ViewAdapter<V>`.
pub trait AnyView: Send {
    /// Begin (or restart) async hydration on the given runtime.
    fn enter(&mut self, handle: &tokio::runtime::Handle);

    /// Non-blocking poll of hydration progress. Call once per frame.
    fn poll(&mut self);

    fn is_loaded(&self) -> bool;

    /// Current visual state: fallback while loading, view once hydrated.
    fn view(&self) -> Element;

    fn update_raw(&mut self, action: BoxedAction);

    fn handle_key(&mut self, key: KeyEvent) -> bool;

    fn drain_actions(&mut self, max: usize) -> usize;

    fn action_rx(&self) -> Receiver<BoxedAction>;

    fn dispatch_box(&self, action: BoxedAction);

    /// Abort the in-flight load task, if any. Idempotent.
    fn abort_load(&mut self);

    fn name(&self) -> &'static str;
}

/// The adapter between the concrete `View<V>` and the erased `AnyView`.
/// Owns the typed action channel, the Tokio handle handle, and the
/// hydration state machine (Idle -> Loading -> Ready | Failed).
pub struct ViewAdapter<V: View> {
    view: Option<V>,
    dispatcher: Dispatcher<V::Action>,
    action_rx: Receiver<BoxedAction>,
    nav: Sender<NavCommand>,
    handle: Option<tokio::runtime::Handle>,
    state: Hydration,
    load_done: Option<tokio::sync::oneshot::Receiver<(V, Result<()>)>>,
    abort: Option<tokio::task::AbortHandle>,
    fallback_el: Option<Element>,
}

enum Hydration {
    Idle,
    Loading,
    Ready,
    Failed,
}

impl<V: View> ViewAdapter<V> {
    pub fn new(view: V, nav: Sender<NavCommand>) -> Self {
        let (dispatcher, action_rx) = channel::<V::Action>(256);
        Self {
            view: Some(view),
            dispatcher,
            action_rx,
            nav,
            handle: None,
            state: Hydration::Idle,
            load_done: None,
            abort: None,
            fallback_el: None,
        }
    }

    fn build_cx(&self) -> Context<V::Action> {
        Context::new(
            self.dispatcher.clone(),
            self.handle
                .clone()
                .expect("Context constructed before enter()"),
            self.nav.clone(),
        )
    }
}

#[async_trait::async_trait]
impl<V: View> AnyView for ViewAdapter<V> {
    fn enter(&mut self, handle: &tokio::runtime::Handle) {
        if matches!(self.state, Hydration::Loading | Hydration::Ready) {
            return;
        }
        self.handle = Some(handle.clone());
        self.state = Hydration::Loading;
        self.fallback_el = self.view.as_ref().map(|v| v.fallback());
        let Some(view) = self.view.take() else { return };
        let (tx, rx) = tokio::sync::oneshot::channel();
        let task = handle.spawn(async move {
            let mut view = view;
            let result = view.load().await;
            let _ = tx.send((view, result));
        });
        self.abort = Some(task.abort_handle());
        self.load_done = Some(rx);
    }

    fn poll(&mut self) {
        if !matches!(self.state, Hydration::Loading) {
            return;
        }
        let Some(rx) = self.load_done.as_mut() else {
            return;
        };
        match rx.try_recv() {
            Ok((view, result)) => {
                self.view = Some(view);
                self.state = match result {
                    Ok(()) => Hydration::Ready,
                    Err(_) => Hydration::Failed,
                };
                self.load_done = None;
                self.abort = None;
            }
            Err(tokio::sync::oneshot::error::TryRecvError::Empty) => {}
            Err(tokio::sync::oneshot::error::TryRecvError::Closed) => {
                // task aborted; view dropped; mark Failed but no view
                self.state = Hydration::Failed;
                self.view = None;
                self.load_done = None;
            }
        }
    }

    fn is_loaded(&self) -> bool {
        matches!(self.state, Hydration::Ready)
    }

    fn view(&self) -> Element {
        match self.state {
            Hydration::Loading | Hydration::Idle => match &self.view {
                Some(v) => v.fallback(),
                None => self
                    .fallback_el
                    .clone()
                    .unwrap_or_else(|| Element::Text(Text::new("loading…"))),
            },
            Hydration::Ready => match &self.view {
                Some(v) => v.view(),
                None => Element::Text(Text::new("…")),
            },
            Hydration::Failed => match &self.view {
                Some(v) => v.fallback(),
                None => Element::Text(Text::new("load failed (aborted)")),
            },
        }
    }

    fn update_raw(&mut self, action: BoxedAction) {
        if let Ok(a) = action.downcast::<V::Action>() {
            if self.view.is_none() {
                return;
            }
            let mut cx = self.build_cx();
            self.view.as_mut().unwrap().update(*a, &mut cx);
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        let mut cx = match &self.view {
            Some(_) => self.build_cx(),
            None => return false,
        };
        match &mut self.view {
            Some(v) => v.handle_key(key, &mut cx),
            None => false,
        }
    }

    fn drain_actions(&mut self, max: usize) -> usize {
        let mut n = 0;
        while n < max {
            match self.action_rx.try_recv() {
                Ok(boxed) => {
                    self.update_raw(boxed);
                    n += 1;
                }
                Err(_) => break,
            }
        }
        n
    }

    fn action_rx(&self) -> Receiver<BoxedAction> {
        self.action_rx.clone()
    }

    fn dispatch_box(&self, action: BoxedAction) {
        self.dispatcher.dispatch_boxed(action);
    }

    fn abort_load(&mut self) {
        if let Some(a) = self.abort.take() {
            a.abort();
        }
        self.load_done = None;
        self.state = Hydration::Failed;
    }

    fn name(&self) -> &'static str {
        std::any::type_name::<V>()
    }
}
