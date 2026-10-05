fn main() {
    veloce_runtime::bootstrap();
    let guard = veloce_runtime::TerminalGuard::init().unwrap();
    assert!(guard.is_active());
    let rt = tokio::runtime::Runtime::new().unwrap();
    veloce_runtime::install_signal_handlers(rt.handle());
    tracing::info!("about to panic");
    panic!("intentional panic inside render path");
}
