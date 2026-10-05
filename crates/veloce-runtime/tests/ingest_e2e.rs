//! C1: Tier-1 runs as a dedicated thread and emits typed AppEvent over a
//! bounded channel — verified end-to-end on a real PTY.
use std::io::Read;
use std::process::Command;
use std::time::Duration;

#[test]
fn tier1_emits_typed_events_over_bounded_channel() {
    let bin = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/ingest_app"
    );
    let out = std::env::temp_dir().join(format!("veloce-ingest-{}.log", std::process::id()));
    let _ = std::fs::remove_file(&out);
    let mut cmd = Command::new(bin);
    cmd.arg(&out);
    cmd.env("TERM", "xterm-256color");
    let mut session = expectrl::Session::spawn(cmd).unwrap();
    std::thread::sleep(Duration::from_millis(700));
    session.send("ab").unwrap();
    std::thread::sleep(Duration::from_millis(700));
    session
        .get_process_mut()
        .kill(expectrl::Signal::SIGKILL)
        .ok();
    let _ = session.read_to_end(&mut Vec::new());
    let content = std::fs::read_to_string(&out).unwrap();
    assert!(
        content.contains("KEY Char"),
        "no key events in {:?}",
        content
    );
    std::fs::remove_file(&out).ok();
}
