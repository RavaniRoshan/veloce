fn main() {
    veloce_runtime::bootstrap();
    let _guard = veloce_runtime::TerminalGuard::init().unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    veloce_runtime::install_signal_handlers(rt.handle());
    println!("ARMED");
    rt.block_on(async { tokio::time::sleep(std::time::Duration::from_secs(60)).await });
}
