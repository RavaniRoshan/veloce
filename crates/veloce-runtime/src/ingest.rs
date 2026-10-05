use std::thread::JoinHandle;
use std::time::Duration;

use crossbeam_channel::Sender;
use crossterm::event::{self, Event};

/// Typed framework events emitted by Tier 1 (I/O ingestion).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
    FocusGained,
    FocusLost,
    Paste(String),
}

impl AppEvent {
    /// Pure mapping from a crossterm event; unit-testable without a TTY.
    pub fn from_crossterm(event: Event) -> Option<Self> {
        match event {
            Event::Key(k) => Some(AppEvent::Key(k)),
            Event::Mouse(m) => Some(AppEvent::Mouse(m)),
            Event::Resize(w, h) => Some(AppEvent::Resize(w, h)),
            Event::FocusGained => Some(AppEvent::FocusGained),
            Event::FocusLost => Some(AppEvent::FocusLost),
            Event::Paste(s) => Some(AppEvent::Paste(s)),
        }
    }
}

/// Tier 1: dedicated OS thread doing blocking `crossterm::event::poll`.
/// Transforms terminal events into `AppEvent` and forwards them over a
/// BOUNDED channel. On a full channel, resize/focus events are coalesced
/// (dropped after combining), while key/mouse/paste apply backpressure by
/// blocking the ingestion thread briefly — input is never silently lost.
pub fn spawn_ingestion_thread(tx: Sender<AppEvent>, poll: Duration) -> JoinHandle<()> {
    std::thread::Builder::new()
        .name("veloce-ingest".into())
        .spawn(move || loop {
            match event::poll(poll) {
                Ok(true) => match event::read() {
                    Ok(ev) => {
                        if let Some(app_event) = AppEvent::from_crossterm(ev) {
                            match &app_event {
                                AppEvent::Resize(..)
                                | AppEvent::FocusGained
                                | AppEvent::FocusLost => {
                                    // Coalesce: drop when the UI thread is behind.
                                    let _ = tx.try_send(app_event);
                                }
                                _ => {
                                    if tx.send(app_event).is_err() {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => continue,
                },
                Ok(false) => continue,
                Err(_) => return,
            }
        })
        .expect("failed to spawn veloce ingestion thread")
}
