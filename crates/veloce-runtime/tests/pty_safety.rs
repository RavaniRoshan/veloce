//! Isolated integration tests proving terminal restoration under panic,
//! normal exit, and signals, on a real PTY (A1, A2, A3).
use std::io::Read;
use std::process::Command;
use std::time::Duration;

#[test]
fn panic_in_app_restores_terminal() {
    let bin = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/panic_app"
    );
    let mut cmd = Command::new(bin);
    cmd.env("TERM", "xterm-256color");
    let mut session = expectrl::Session::spawn(cmd).unwrap();
    let mut out = String::new();
    let _ = session.read_to_string(&mut out);
    assert!(
        out.contains("\u{1b}[?1049l"),
        "leave-alt-screen missing after panic; got: {:?}",
        out
    );
    assert!(out.contains("intentional panic"), "panic message missing");
}

#[test]
fn normal_exit_restores_terminal() {
    let bin = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/restore_app"
    );
    let mut cmd = Command::new(bin);
    cmd.env("TERM", "xterm-256color");
    let mut session = expectrl::Session::spawn(cmd).unwrap();
    let mut out = String::new();
    let _ = session.read_to_string(&mut out);
    assert!(out.contains("\u{1b}[?1049h"), "expected alt-screen enter");
    assert!(
        out.contains("\u{1b}[?1049l"),
        "expected alt-screen leave on drop"
    );
    assert!(out.contains("READY") && out.contains("DONE"));
}

#[test]
fn sigterm_restores_terminal() {
    let bin = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/signal_app"
    );
    let mut cmd = Command::new(bin);
    cmd.env("TERM", "xterm-256color");
    let mut session = expectrl::Session::spawn(cmd).unwrap();
    std::thread::sleep(Duration::from_millis(500));
    session
        .get_process_mut()
        .kill(expectrl::Signal::SIGTERM)
        .unwrap();
    let mut out = String::new();
    let _ = session.read_to_string(&mut out);
    assert!(
        out.contains("\u{1b}[?1049l"),
        "leave-alt-screen missing after SIGTERM"
    );
}

#[test]
fn sigint_restores_terminal() {
    let bin = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/signal_app"
    );
    let mut cmd = Command::new(bin);
    cmd.env("TERM", "xterm-256color");
    let mut session = expectrl::Session::spawn(cmd).unwrap();
    std::thread::sleep(Duration::from_millis(500));
    session
        .get_process_mut()
        .kill(expectrl::Signal::SIGINT)
        .unwrap();
    let mut out = String::new();
    let _ = session.read_to_string(&mut out);
    assert!(
        out.contains("\u{1b}[?1049l"),
        "leave-alt-screen missing after SIGINT"
    );
}
