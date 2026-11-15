use std::any::Any;

use crossbeam_channel::{Receiver, Sender};

/// Type-erased action payload traveling on the wire. Typed `Dispatcher<A>`
/// boxes actions; the route adapter downcasts them back. No view action
/// type ever appears in the router's stored types.
pub type BoxedAction = Box<dyn Any + Send>;

/// Cloneable, typed action sink. Background tasks (and `Context::dispatch`)
/// use it to feed actions back to the UI thread. Bounded: when full,
/// producers block (backpressure), never a lock.
pub struct Dispatcher<A> {
    tx: Sender<BoxedAction>,
    _marker: std::marker::PhantomData<A>,
}

impl<A> Clone for Dispatcher<A> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A: Send + 'static> Dispatcher<A> {
    pub fn dispatch(&self, action: A) {
        // If the UI thread has gone away, there is nowhere to deliver;
        // dropping is the only sane non-panicking option.
        let _ = self.tx.send(Box::new(action));
    }

    pub fn dispatch_boxed(&self, action: BoxedAction) {
        let _ = self.tx.send(action);
    }

    pub fn try_dispatch(&self, action: A) -> Result<(), A> {
        self.tx.try_send(Box::new(action)).map_err(|e| match e {
            crossbeam_channel::TrySendError::Full(a) => *a.downcast().unwrap(),
            crossbeam_channel::TrySendError::Disconnected(a) => *a.downcast().unwrap(),
        })
    }

    /// Non-blocking dispatch that discards the action when the bounded
    /// channel is full. Useful for lossy, high-frequency events (hover
    /// moves, tick heartbeats) where backpressure on the producer thread
    /// would be worse than dropping a stale event.
    pub fn try_dispatch_or_drop(&self, action: A) -> bool {
        self.try_dispatch(action).is_ok()
    }

    /// Returns `true` once the UI-thread receiver has been dropped, meaning
    /// further dispatches are silently discarded. Lets long-running tasks
    /// early-exit instead of doing work nobody will observe. Probes with an
    /// empty boxed payload so no spurious action reaches the view when the
    /// channel is merely full (Full != disconnected).
    pub fn is_disconnected(&self) -> bool {
        match self.tx.try_send(Box::new(()) as BoxedAction) {
            Err(crossbeam_channel::TrySendError::Disconnected(_)) => true,
            Ok(()) | Err(crossbeam_channel::TrySendError::Full(_)) => false,
        }
    }
}

/// Creates a bounded (Dispatcher, Receiver) pair. The receiver carries
/// type-erased actions; the owning route adapter downcasts them.
pub fn channel<A: Send + 'static>(capacity: usize) -> (Dispatcher<A>, Receiver<BoxedAction>) {
    let (tx, rx) = crossbeam_channel::bounded(capacity);
    (
        Dispatcher {
            tx,
            _marker: std::marker::PhantomData,
        },
        rx,
    )
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

impl<A: Send + 'static> Context<A> {
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

#[cfg(test)]
mod dispatcher_tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Act(u32);

    #[test]
    fn try_dispatch_or_drop_delivers_when_capacity_allows() {
        let (d, rx) = channel::<Act>(2);
        assert!(d.try_dispatch_or_drop(Act(1)));
        assert!(d.try_dispatch_or_drop(Act(2)));
        assert_eq!(rx.len(), 2);
    }

    #[test]
    fn try_dispatch_or_drop_discards_when_full() {
        let (d, _rx) = channel::<Act>(1);
        assert!(d.try_dispatch_or_drop(Act(1)));
        // Channel is now full: the second action is dropped, not blocked.
        assert!(!d.try_dispatch_or_drop(Act(2)));
    }

    #[test]
    fn is_disconnected_after_receiver_drop() {
        let (d, rx) = channel::<Act>(4);
        assert!(!d.is_disconnected());
        drop(rx);
        assert!(d.is_disconnected());
        // A disconnected dispatcher reports drops rather than blocking.
        assert!(!d.try_dispatch_or_drop(Act(9)));
    }
}
