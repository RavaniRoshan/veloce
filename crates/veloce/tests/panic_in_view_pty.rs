//! A2 (strict): a panic raised INSIDE view() and INSIDE update() during a
//! real VeloceApp frame loop must restore the terminal every time.
use std::io::Read;
use std::process::Command;
use std::time::Duration;

const RESTORED: &str = "\u{1b}[?1049l";

fn run_and_capture(send: &str) -> String {
    let bin = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/panic_view_app"
    );
    let mut cmd = Command::new(bin);
    cmd.env("TERM", "xterm-256color");
    let mut session = expectrl::Session::spawn(cmd).unwrap();
    std::thread::sleep(Duration::from_millis(800));
    if !send.is_empty() {
        session.send(send).unwrap();
        std::thread::sleep(Duration::from_millis(800));
    }
    session
        .get_process_mut()
        .kill(expectrl::Signal::SIGKILL)
        .ok();
    let mut out = String::new();
    let _ = session.read_to_string(&mut out);
    out
}

#[test]
fn panic_inside_view_restores_terminal() {
    let out = run_and_capture("v");
    eprintln!("--- panic in view() captured ---\n{out}\n---");
    assert!(
        out.contains(RESTORED),
        "terminal not restored after panic in view()"
    );
    assert!(
        out.contains("intentional panic inside view()"),
        "panic message not surfaced"
    );
}

#[test]
fn panic_inside_update_restores_terminal() {
    let out = run_and_capture("u");
    eprintln!("--- panic in update() captured ---\n{out}\n---");
    assert!(
        out.contains(RESTORED),
        "terminal not restored after panic in update()"
    );
    assert!(
        out.contains("intentional panic inside update()"),
        "panic message not surfaced"
    );
}
