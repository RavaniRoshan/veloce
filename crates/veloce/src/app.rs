use anyhow::Result;

use veloce_core::View;
use veloce_router::{LayoutWrapper, Router};
use veloce_runtime::{spawn_ingestion_thread, tick_interval, AppEvent, TerminalGuard};

/// The framework's front door: owns the ingestion thread, the router, and
/// the 60 FPS frame pipeline. Users never touch channels, the Runtime, or
/// thread internals; async is driven from user code's `#[tokio::main]`.
pub struct VeloceApp {
    router: Router,
    started: bool,
}

impl VeloceApp {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            started: false,
        }
    }

    pub fn route<V>(&mut self, path: &str, view: V) -> &mut Self
    where
        V: View + Clone,
    {
        self.router.route(path, view);
        self
    }

    pub fn root(&mut self, path: &str) -> &mut Self {
        self.router.root(path);
        self
    }

    pub fn set_wrapper(&mut self, wrapper: LayoutWrapper) -> &mut Self {
        self.router.set_wrapper(wrapper);
        self
    }

    /// Runs the main loop until 'q' (or Ctrl+C / signal-driven exit).
    pub async fn run(&mut self) -> Result<()> {
        veloce_runtime::bootstrap();
        veloce_runtime::init_tracing(None)?;
        let _guard = TerminalGuard::init()?;
        if self.router.top().is_none() {
            // default: first registered route is the root
            self.router.root("/");
            self.started = true;
        }
        let handle = tokio::runtime::Handle::current();
        veloce_runtime::install_signal_handlers(&handle);
        let (event_tx, event_rx) = crossbeam_channel::bounded(256);
        let _ingest = spawn_ingestion_thread(event_tx, std::time::Duration::from_millis(5));
        let mut interval = tick_interval();
        let mut terminal =
            ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;

        loop {
            // input events first (highest priority)
            while let Ok(ev) = event_rx.try_recv() {
                match ev {
                    AppEvent::Key(k) => {
                        if k.code == crossterm::event::KeyCode::Char('q') {
                            return Ok(());
                        }
                        self.router.handle_key(k);
                    }
                    _ => {}
                }
            }
            self.router.drain_command_queue();
            for route in self.router.stack.iter_mut() {
                route.view.drain_actions(64);
            }
            self.router.step(&handle);
            let _ =
                terminal.draw(|f| veloce_render::render(&self.router.element(), f.buffer_mut()));
            interval.tick().await;
        }
    }
}

impl Default for VeloceApp {
    fn default() -> Self {
        Self::new()
    }
}
