//! F5: settings form + streaming log viewer composed only from primitives.
use std::io::Read;
use std::process::Command;
use std::time::Duration;

#[test]
fn settings_form_and_log_viewer_via_primitives() {
    let bin = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/settings_log_viewer"
    );
    let mut cmd = Command::new(bin);
    cmd.env("TERM", "xterm-256color");
    let mut session = expectrl::Session::spawn(cmd).unwrap();
    std::thread::sleep(Duration::from_millis(800));
    session.send("volo").unwrap();
    std::thread::sleep(Duration::from_millis(500));
    session.send("\t").unwrap();
    session.send("example.com").unwrap();
    std::thread::sleep(Duration::from_millis(800));
    session.send("q").unwrap();
    let mut out = String::new();
    let _ = session.read_to_string(&mut out);
    eprintln!("--- settings PTY output ---\n{out}\n-----------------");
    assert!(out.contains("Veloce Settings"), "chrome missing");
    assert!(out.contains("volo"), "name field missing typed text");
    assert!(out.contains("example.co"), "host field missing typed text");
    assert!(
        out.contains("dashboard") && out.contains("watching"),
        "log viewer not hydrated"
    );
    assert!(out.contains("\u{1b}[?1049l"), "terminal not restored");
}
