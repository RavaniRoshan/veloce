use std::sync::Once;

use thiserror::Error;

pub mod drain;
pub mod guard;
pub mod hooks;
pub mod ingest;
pub mod logging;
pub mod signals;
pub mod stats;
pub mod tick;

pub use drain::{drain_prioritized, Drained};
pub use guard::TerminalGuard;
pub use ingest::{spawn_ingestion_thread, AppEvent};
pub use logging::init_tracing;
pub use signals::install_signal_handlers;
pub use stats::{frame_stats, layout_cache_enabled, record_frame, set_layout_cache, FrameStats};
pub use tick::tick_interval;

static INIT: Once = Once::new();

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("terminal error: {0}")]
    Terminal(#[from] std::io::Error),
}

/// Idempotent process bootstrap: installs panic hook and signal handlers once.
pub fn bootstrap() {
    INIT.call_once(|| {
        hooks::install_panic_hook();
    });
}
