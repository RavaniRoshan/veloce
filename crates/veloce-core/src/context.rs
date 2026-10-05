use crossbeam_channel::{Receiver, Sender};

/// Cloneable, typed action sink. Background tasks (and `Context::dispatch`)
/// use it to feed actions back to the UI thread. Bounded: when full,
/// producers block (backpressure), never a lock.
pub struct Dispatcher<A> {
    tx: Sender<A>,
}

impl<A> Clone for Dispatcher<A> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<A> Dispatcher<A> {
    pub fn dispatch(&self, action: A) {
        // If the UI thread has gone away, there is nowhere to deliver;
        // dropping is the only sane non-panicking option.
        let _ = self.tx.send(action);
    }

    pub fn try_dispatch(&self, action: A) -> Result<(), A> {
        self.tx.try_send(action).map_err(|e| match e {
            crossbeam_channel::TrySendError::Full(a) => a,
            crossbeam_channel::TrySendError::Disconnected(a) => a,
        })
    }
}

/// Creates a bounded (Dispatcher, Receiver) pair.
pub fn channel<A>(capacity: usize) -> (Dispatcher<A>, Receiver<A>) {
    let (tx, rx) = crossbeam_channel::bounded(capacity);
    (Dispatcher { tx }, rx)
}

/// Context handed to `View::update` and event hooks. Wires navigation,
/// async spawn, and the action dispatcher. The Tokio handle and router
/// internals live in private fields and never appear in method signatures.
pub struct Context<A> {
    dispatcher: Dispatcher<A>,
    handle: tokio::runtime::Handle,
    nav: Sender<NavCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavCommand {
    Push(String),
    Pop,
    Replace(String),
}

impl<A> Context<A> {
    /// Used by the framework runtime only. Public because crates are split;
    /// it is the single place that touches the Tokio handle.
    pub fn new(
        dispatcher: Dispatcher<A>,
        handle: tokio::runtime::Handle,
        nav: Sender<NavCommand>,
    ) -> Self {
        Self {
            dispatcher,
            handle,
            nav,
        }
    }

    /// Enqueue an action back into this view's state machine.
    pub fn dispatch(&mut self, action: A) {
        self.dispatcher.dispatch(action);
    }

    /// The cloneable handle background tasks use to feed actions back.
    pub fn dispatcher(&self) -> Dispatcher<A> {
        self.dispatcher.clone()
    }

    /// Spawn a fire-and-forget async task on the owned Tokio runtime.
    pub fn spawn<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.handle.spawn(future);
    }

    pub fn navigate(&mut self, path: impl Into<String>) {
        let _ = self.nav.send(NavCommand::Push(path.into()));
    }

    pub fn back(&mut self) {
        let _ = self.nav.send(NavCommand::Pop);
    }

    pub fn replace(&mut self, path: impl Into<String>) {
        let _ = self.nav.send(NavCommand::Replace(path.into()));
    }
}
