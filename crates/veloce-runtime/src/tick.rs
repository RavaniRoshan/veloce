use std::time::Duration;

/// 60 FPS ticker used by Tier 3 for periodic redraws.
/// Must be created inside a Tokio runtime context (timer driver).
pub fn tick_interval() -> tokio::time::Interval {
    let mut interval = tokio::time::interval(Duration::from_millis(16));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    interval
}
