fn main() {
    veloce_runtime::bootstrap();
    {
        let _guard = veloce_runtime::TerminalGuard::init().unwrap();
        println!("READY");
    }
    println!("DONE");
}
