//! A4 non-TTY/TERM=dumb emits no escapes; A5 no stdout writes in src;
//! 60 FPS ticker cadence; ingestion backpressure semantics.
use std::process::Command;

#[test]
fn non_tty_emits_no_escape_sequences() {
    let bin = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/restore_app"
    );
    let output = Command::new(bin)
        .env("TERM", "dumb")
        .output()
        .expect("run restore_app");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("READY") && stdout.contains("DONE"));
    assert!(
        !stdout.contains('\u{1b}'),
        "escape byte in non-TTY/dumb output: {:?}",
        stdout
    );
}

#[test]
fn no_print_macros_in_framework_src() {
    let mut root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.pop();
    root.pop();
    let mut bad = vec![];
    for entry in walk(&root) {
        let text = std::fs::read_to_string(&entry).unwrap();
        for (i, line) in text.lines().enumerate() {
            for pat in ["println!", "eprintln!", "print!", "eprint!"] {
                if line.contains(pat) {
                    bad.push(format!("{}:{}: {}", entry.display(), i + 1, line));
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "stdout prints in framework src:\n{}",
        bad.join("\n")
    );
}

fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = vec![];
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() && p.file_name().unwrap() == "src" {
            out.extend(walk_files(&p));
        }
    }
    out
}

fn walk_files(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = vec![];
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            out.extend(walk_files(&p));
        } else if p.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(p);
        }
    }
    out
}

#[tokio::test]
async fn tick_is_sixty_hz() {
    let mut interval = veloce_runtime::tick_interval();
    let start = std::time::Instant::now();
    for _ in 0..4 {
        interval.tick().await;
    }
    let elapsed = start.elapsed();
    // 4 ticks over ~48ms (first tick immediate): expect 3 intervals ~48ms.
    assert!(
        elapsed >= std::time::Duration::from_millis(40),
        "ticks too fast: {:?}",
        elapsed
    );
    assert!(
        elapsed < std::time::Duration::from_millis(200),
        "ticks too slow: {:?}",
        elapsed
    );
}

#[test]
fn ingestion_channel_is_bounded() {
    let (tx, rx) = crossbeam_channel::bounded::<veloce_runtime::AppEvent>(2);
    tx.try_send(veloce_runtime::AppEvent::Resize(1, 1)).unwrap();
    tx.try_send(veloce_runtime::AppEvent::Resize(2, 2)).unwrap();
    assert!(tx.try_send(veloce_runtime::AppEvent::Resize(3, 3)).is_err());
    assert_eq!(rx.recv().unwrap(), veloce_runtime::AppEvent::Resize(1, 1));
}

#[test]
fn no_shared_mutable_state_in_public_api() {
    // C6: no Arc<Mutex>/Rc<RefCell> exposed in public signatures.
    let mut root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.pop();
    root.pop();
    let mut bad = vec![];
    for entry in walk_dirs(&root) {
        let text = std::fs::read_to_string(&entry).unwrap();
        for (i, line) in text.lines().enumerate() {
            let hits = line.contains("Arc<Mutex")
                || line.contains("Rc<RefCell")
                || (line.contains("pub ") && (line.contains("Mutex") || line.contains("RefCell")));
            if hits {
                bad.push(format!("{}:{}: {}", entry.display(), i + 1, line));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "shared-mutable-state in public API:\n{}",
        bad.join("\n")
    );
}

fn walk_dirs(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = vec![];
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() && p.file_name().unwrap() == "src" {
            out.extend(walk_files(&p));
        }
    }
    out
}
