//! Measured probes (A6, A7). Values recorded in docs/CHECKLIST.md.
use std::io::Write;
use std::time::Instant;

fn rss_kib() -> usize {
    let status = std::fs::read_to_string("/proc/self/status").unwrap();
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            let kb: usize = rest.trim().trim_end_matches(" kB").parse().unwrap();
            return kb;
        }
    }
    panic!("VmRSS not found")
}

#[test]
fn cold_boot_to_first_frame_under_15ms() {
    let start = Instant::now();
    veloce_runtime::bootstrap();
    let _guard = veloce_runtime::TerminalGuard::init().unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut interval = veloce_runtime::tick_interval();
        interval.tick().await;
    });
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, 24)).unwrap();
    terminal
        .draw(|f| f.render_widget(ratatui::widgets::Block::default(), f.area()))
        .unwrap();
    let elapsed = start.elapsed();
    println!("COLD_BOOT_MS={}", elapsed.as_secs_f64() * 1000.0);
    std::io::stdout().flush().unwrap();
    assert!(
        elapsed.as_millis() < 15,
        "cold boot {}ms >= 15ms",
        elapsed.as_millis()
    );
}

#[test]
fn idle_memory_under_12mb() {
    veloce_runtime::bootstrap();
    let _guard = veloce_runtime::TerminalGuard::init().unwrap();
    let _rt = tokio::runtime::Runtime::new().unwrap();
    let rss = rss_kib();
    println!("IDLE_RSS_KIB={}", rss);
    std::io::stdout().flush().unwrap();
    assert!(rss < 12 * 1024, "idle RSS {} KiB >= 12 MiB", rss);
}
