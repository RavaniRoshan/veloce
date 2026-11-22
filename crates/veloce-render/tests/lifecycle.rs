use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use veloce_core::{AnyView, Context, Element, Text, View, ViewAdapter};

#[derive(Default)]
struct TestView {
    name: String,
    deploys: usize,
}

#[derive(Clone)]
enum TestAction {
    Deploy,
}

#[async_trait::async_trait]
impl View for TestView {
    type Action = TestAction;

    async fn load(&mut self) -> Result<()> {
        tokio::time::sleep(Duration::from_millis(60)).await;
        self.name = "auth-service-v2".to_string();
        Ok(())
    }

    fn fallback(&self) -> Element {
        Element::Text(Text::new("FALLBACK"))
    }

    fn update(&mut self, action: TestAction, _cx: &mut Context<TestAction>) {
        match action {
            TestAction::Deploy => self.deploys += 1,
        }
    }

    fn view(&self) -> Element {
        Element::Text(Text::new(format!(
            "name={} deploys={}",
            self.name, self.deploys
        )))
    }

    fn handle_key(
        &mut self,
        key: crossterm::event::KeyEvent,
        _cx: &mut Context<TestAction>,
    ) -> bool {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char('q') => true, // consume
            _ => false,
        }
    }
}

#[test]
fn hydration_fallback_then_ready_no_flicker() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, _rx) = crossbeam_channel::bounded(8);
    let mut adapter = ViewAdapter::new(TestView::default(), tx);

    // immediate: fallback shown, hydration pending
    adapter.enter(rt.handle());
    let el = adapter.view();
    let grid = veloce_render::render_to_string(&el, 20, 3);
    assert!(grid.contains("FALLBACK"), "not loading state: {grid}");

    // not yet ready
    adapter.poll();
    assert!(!adapter.is_loaded());

    std::thread::sleep(Duration::from_millis(120));
    adapter.poll();
    assert!(adapter.is_loaded());
    let el = adapter.view();
    let grid = veloce_render::render_to_string(&el, 20, 3);
    assert!(grid.contains("auth-service-v2"), "not hydrated: {grid}");

    // subsequent polls never revert to fallback (no flicker)
    adapter.poll();
    adapter.poll();
    let el = adapter.view();
    let grid = veloce_render::render_to_string(&el, 20, 3);
    assert!(grid.contains("auth-service-v2"));
}

#[test]
fn dispatch_roundtrip_updates_state() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, _rx) = crossbeam_channel::bounded(8);
    let mut adapter = ViewAdapter::new(TestView::default(), tx);
    adapter.enter(rt.handle());
    std::thread::sleep(Duration::from_millis(120));
    adapter.poll();

    adapter.dispatch_box(Box::new(TestAction::Deploy));
    let action_rx = adapter.action_rx();
    let boxed = action_rx.recv().unwrap();
    adapter.update_raw(boxed);
    let grid = veloce_render::render_to_string(&adapter.view(), 30, 3);
    assert!(grid.contains("deploys=1"), "grid: {grid}");
}

#[test]
fn abort_mid_load_cancels_task_and_drops_view() {
    let drops = Arc::new(AtomicUsize::new(0));
    let drops2 = drops.clone();
    struct D(Arc<AtomicUsize>);
    #[async_trait::async_trait]
    impl View for D {
        type Action = TestAction;
        async fn load(&mut self) -> Result<()> {
            tokio::time::sleep(Duration::from_secs(30)).await;
            Ok(())
        }
        fn update(&mut self, _a: TestAction, _cx: &mut Context<TestAction>) {}
        fn view(&self) -> Element {
            Element::Text(Text::new("D"))
        }
    }
    impl Drop for D {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, _rx) = crossbeam_channel::bounded(8);
    let mut adapter = ViewAdapter::new(D(drops2), tx);
    adapter.enter(rt.handle());
    std::thread::sleep(Duration::from_millis(50));
    adapter.abort_load();
    std::thread::sleep(Duration::from_millis(50));
    assert!(!adapter.is_loaded());
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn handle_key_consumed_before_focus_manager() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, _rx) = crossbeam_channel::bounded(8);
    let mut adapter = ViewAdapter::new(TestView::default(), tx);
    adapter.enter(rt.handle());
    std::thread::sleep(Duration::from_millis(120));
    adapter.poll();
    let focus_manager_calls = Arc::new(AtomicUsize::new(0));
    let key = crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('q'),
        crossterm::event::KeyModifiers::NONE,
    );
    let consumed = adapter.handle_key(key);
    if !consumed {
        focus_manager_calls.fetch_add(1, Ordering::SeqCst);
    }
    assert!(consumed);
    assert_eq!(focus_manager_calls.load(Ordering::SeqCst), 0);
}

#[test]
fn any_view_is_dyn_safe() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, _rx) = crossbeam_channel::bounded(8);
    let mut views: Vec<Box<dyn AnyView>> =
        vec![Box::new(ViewAdapter::new(TestView::default(), tx))];
    views[0].enter(rt.handle());
    std::thread::sleep(Duration::from_millis(120));
    views[0].poll();
    assert!(views[0].is_loaded());
    assert!(views[0].name().contains("TestView"));
}
