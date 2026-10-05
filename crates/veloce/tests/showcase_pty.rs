//! G3: the AI coding-agent harness dashboard showcase runs end to end.
use std::io::Read;
use std::process::Command;
use std::time::Duration;

#[test]
fn agent_dashboard_streams_mock_tool_executions() {
    let bin = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/agent_dashboard"
    );
    let mut cmd = Command::new(bin);
    cmd.env("TERM", "xterm-256color");
    let mut session = expectrl::Session::spawn(cmd).unwrap();
    std::thread::sleep(Duration::from_millis(800));
    session.send("r").unwrap();
    std::thread::sleep(Duration::from_millis(1000));
    session.send("q").unwrap();
    let mut out = String::new();
    let _ = session.read_to_string(&mut out);
    eprintln!("--- showcase PTY output (truncated) ---");
    eprintln!("{}", &out[out.len().saturating_sub(1500)..]);
    assert!(out.contains("agent:"), "header missing");
    assert!(
        out.contains("tool") && (out.contains("src/main.rs") || out.contains("cargo")),
        "tool stream missing"
    );
    assert!(out.contains("\u{1b}[?1049l"), "terminal not restored");
}
