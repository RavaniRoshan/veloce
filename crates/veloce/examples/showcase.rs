//! The 15-second demo: live frame timings, a real layout-cache switch,
//! streaming rows, and a panic that restores the terminal.
//!
//! ```bash
//! cargo run -p veloce-tui --example showcase
//! ```
//!
//! Keys: `c` toggles the layout cache, `r` adds rows, `v` panics inside
//! `update()` (the runtime must restore the terminal), `q` quits.

use std::time::Duration;

use anyhow::Result;
use ratatui::style::Color;
use veloce_tui::prelude::*;

#[derive(Clone)]
enum Action {
    AddRows,
    Panic,
}

#[derive(Clone)]
struct Showcase {
    hydrated: bool,
    rows: usize,
}

impl Default for Showcase {
    fn default() -> Self {
        Self {
            hydrated: false,
            rows: 6,
        }
    }
}

fn telemetry_row(i: usize) -> String {
    let services = ["router", "layout", "hydrate", "runtime", "render"];
    let service = services[i % services.len()];
    let ms = 3 + (i * 7 % 21);
    format!(
        "  {:>4}  {:<8}  {:<9}  {:>3} ms  {}",
        i,
        service,
        if i.is_multiple_of(3) {
            "cached"
        } else {
            "fresh "
        },
        ms,
        if i.is_multiple_of(4) { "ok" } else { "healthy" }
    )
}

#[async_trait::async_trait]
impl View for Showcase {
    type Action = Action;

    async fn load(&mut self) -> Result<()> {
        // A short hydration window makes the framework's fallback visible.
        tokio::time::sleep(Duration::from_millis(350)).await;
        self.hydrated = true;
        Ok(())
    }

    fn fallback(&self) -> Element {
        Flex::column()
            .padding(2)
            .gap(1)
            .child(Text::new("VELOCE / SHOWCASE").bold().color(Color::Cyan))
            .child(Text::new("hydrating…").color(Color::Yellow))
            .into_element()
    }

    fn update(&mut self, action: Action, _cx: &mut Context<Action>) {
        match action {
            Action::AddRows => self.rows = (self.rows + 8).min(40),
            Action::Panic => panic!("intentional panic in update(): terminal must be restored"),
        }
    }

    fn view(&self) -> Element {
        let stats = frame_stats();
        let cached = layout_cache_enabled();
        let cache_label = if cached { "cache ON " } else { "cache OFF" };
        let cache_color = if cached { Color::Green } else { Color::Yellow };

        Flex::column()
            .gap(1)
            .padding(1)
            .border(BorderStyle::Rounded)
            .child(
                Flex::row()
                    .padding(1)
                    .child(Text::new("VELOCE / SHOWCASE").bold().color(Color::Cyan))
                    .child(Spacer::grow())
                    .child(Text::new("v0.1.0").color(Color::DarkGray)),
            )
            .child(
                Text::new(format!(
                    "frames {:>4} | last {:>5} us | avg {:>5} us | {cache_label}",
                    stats.frames, stats.last_micros, stats.average_micros
                ))
                .color(cache_color),
            )
            .child(
                Text::new(
                    "cache reuses clean taffy trees: 2,000 children 2.25 ms -> 160 us (measured)",
                )
                .color(Color::DarkGray),
            )
            .child(
                Flex::column()
                    .flex_grow(1.0)
                    .border(BorderStyle::Square)
                    .child(ScrollView::new().flex_grow(1.0).children(
                        (0..self.rows).map(|i| Text::new(telemetry_row(i)).into_element()),
                    )),
            )
            .child(
                Text::new(if self.hydrated {
                    "c cache on/off    r add rows    v panic (restores terminal)    q quit"
                } else {
                    "loading telemetry…"
                })
                .color(Color::DarkGray),
            )
            .into_element()
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent, cx: &mut Context<Action>) -> bool {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char('c') => {
                set_layout_cache(!layout_cache_enabled());
                true
            }
            KeyCode::Char('r') => {
                cx.dispatch(Action::AddRows);
                true
            }
            KeyCode::Char('v') => {
                cx.dispatch(Action::Panic);
                true
            }
            _ => false,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    VeloceApp::new().route("/", Showcase::default()).run().await
}
