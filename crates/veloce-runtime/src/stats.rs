//! Process-wide frame statistics and the layout-cache switch.
//!
//! The numbers are measured by the render loop, not estimated. They exist so
//! users can verify the framework's performance claims with a live counter
//! instead of trusting a benchmark table.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;

/// A snapshot of frame timings since process start.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FrameStats {
    /// Wall-clock time spent in the last `terminal.draw` call, in microseconds.
    pub last_micros: u64,
    /// Mean time across all recorded frames, in microseconds.
    pub average_micros: u64,
    /// Number of frames recorded so far.
    pub frames: u64,
}

impl FrameStats {
    /// Builds a snapshot from raw microsecond samples. Used by tests and by
    /// tools that replay recorded frames.
    pub fn from_samples(samples: &[u64]) -> Self {
        let frames = samples.len() as u64;
        let total: u64 = samples.iter().copied().sum();
        Self {
            last_micros: samples.last().copied().unwrap_or(0),
            average_micros: total.checked_div(frames).unwrap_or(0),
            frames,
        }
    }
}

static LAST_MICROS: AtomicU64 = AtomicU64::new(0);
static TOTAL_MICROS: AtomicU64 = AtomicU64::new(0);
static FRAMES: AtomicU64 = AtomicU64::new(0);
static CACHE_ENABLED: AtomicBool = AtomicBool::new(true);

/// Records one completed frame. Called by `VeloceApp::run`, not by user code.
pub fn record_frame(elapsed: Duration) {
    let micros = elapsed.as_micros().min(u64::MAX as u128) as u64;
    LAST_MICROS.store(micros, Ordering::Relaxed);
    TOTAL_MICROS.fetch_add(micros, Ordering::Relaxed);
    FRAMES.fetch_add(1, Ordering::Relaxed);
}

/// Returns a snapshot of the frame timings recorded so far.
pub fn frame_stats() -> FrameStats {
    let frames = FRAMES.load(Ordering::Relaxed);
    let total = TOTAL_MICROS.load(Ordering::Relaxed);
    FrameStats {
        last_micros: LAST_MICROS.load(Ordering::Relaxed),
        average_micros: total.checked_div(frames).unwrap_or(0),
        frames,
    }
}

/// Enables or disables reuse of the last layout tree at runtime.
///
/// Disabling the cache makes every frame run a full taffy layout. This is how
/// the `showcase` example demonstrates the cache's effect live.
pub fn set_layout_cache(enabled: bool) {
    CACHE_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Whether frames currently reuse the last clean layout tree.
pub fn layout_cache_enabled() -> bool {
    CACHE_ENABLED.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_from_samples_reports_last_and_average() {
        let stats = FrameStats::from_samples(&[100, 200, 300]);
        assert_eq!(stats.frames, 3);
        assert_eq!(stats.last_micros, 300);
        assert_eq!(stats.average_micros, 200);
    }

    #[test]
    fn stats_from_empty_samples_are_zeroed() {
        assert_eq!(FrameStats::from_samples(&[]), FrameStats::default());
    }

    #[test]
    fn cache_toggle_round_trips() {
        let original = layout_cache_enabled();
        set_layout_cache(!original);
        assert_eq!(layout_cache_enabled(), !original);
        set_layout_cache(original);
        assert_eq!(layout_cache_enabled(), original);
    }
}
