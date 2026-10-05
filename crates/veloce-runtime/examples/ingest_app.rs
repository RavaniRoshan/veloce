use std::io::Write;
fn main() {
    let out_path = std::env::args().nth(1).expect("out path arg");
    veloce_runtime::bootstrap();
    let _guard = veloce_runtime::TerminalGuard::init().unwrap();
    let (tx, rx) = crossbeam_channel::bounded(256);
    let _j = veloce_runtime::spawn_ingestion_thread(tx, std::time::Duration::from_millis(5));
    let mut count = 0;
    for ev in rx.iter() {
        count += 1;
        let line = match ev {
            veloce_runtime::AppEvent::Key(k) => format!("KEY {:?}\n", k.code),
            veloce_runtime::AppEvent::Resize(w, h) => format!("RESIZE {} {}\n", w, h),
            _ => "OTHER\n".to_string(),
        };
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&out_path)
            .unwrap()
            .write_all(line.as_bytes())
            .unwrap();
        if count >= 8 {
            break;
        }
    }
}
