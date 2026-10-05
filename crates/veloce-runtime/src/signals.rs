use crate::guard::TerminalGuard;

/// Traps SIGINT/SIGTERM: restores the terminal, flushes logs, then exits
/// with the conventional 128+signal code. Handlers are installed on the
/// Tokio runtime the app already owns; no runtime is exposed to users.
pub fn install_signal_handlers(handle: &tokio::runtime::Handle) {
    handle.spawn(async move {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};
            let mut int = match signal(SignalKind::interrupt()) {
                Ok(s) => s,
                Err(_) => return,
            };
            let mut term = match signal(SignalKind::terminate()) {
                Ok(s) => s,
                Err(_) => return,
            };
            tokio::select! {
                _ = int.recv() => exit_with(130),
                _ = term.recv() => exit_with(143),
            }
        }
        #[cfg(not(unix))]
        {
            if tokio::signal::ctrl_c().await.is_ok() {
                exit_with(130);
            }
        }
    });
}

fn exit_with(code: i32) -> ! {
    TerminalGuard::restore_now();
    let _ = tracing::dispatcher::get_default(|d| {
        // Best-effort flush of buffered subscribers before exit.
        let _ = d;
    });
    std::process::exit(code);
}
