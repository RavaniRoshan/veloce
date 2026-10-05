//! The reference API example driven on a real PTY: proves VeloceApp runs
//! a multi-view app, hydrates, reacts to keys, restores the terminal.
use std::io::Read;
use std::process::Command;
use std::time::Duration;

#[test]
fn reference_app_hydrates_deploys_and_restores() {
    let bin = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/debug/examples/reference_app");
    let mut cmd = Command::new(bin);
    cmd.env("TERM", "xterm-256color");
    let mut session = expectrl::Session::spawn(cmd).unwrap();
    std::thread::sleep(Duration::from_millis(800));
    session.send("d").unwrap();
    std::thread::sleep(Duration::from_millis(1200));
    session.send("q").unwrap();
    let mut out = String::new();
    let _ = session.read_to_string(&mut out);
    eprintln!("--- captured PTY output ---\n{out}\n---------------------------");
    assert!(out.contains("Service: auth-service-v2"), "hydrated service name missing");
    assert!(out.contains("Deploying") || out.contains("Live"), "deploy flow did not run");
    assert!(out.contains("\u{1b}[?1049l"), "terminal not restored");
}
