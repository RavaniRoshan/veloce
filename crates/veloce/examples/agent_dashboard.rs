//! G3 showcase: an AI coding-agent harness dashboard streaming mock tool
//! executions — built end-to-end on Veloce primitives.
use anyhow::Result;
use veloce::prelude::*;

#[derive(Default, Clone)]
struct AgentDash {
    lines: Vec<String>,
    running: bool,
}

#[derive(Clone)]
enum Act {
    Line(String),
    Done,
}

#[async_trait::async_trait]
impl View for AgentDash {
    type Action = Act;

    async fn load(&mut self) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, action: Act, _cx: &mut Context<Act>) {
        match action {
            Act::Line(l) => self.lines.push(l),
            Act::Done => self.running = false,
        }
    }

    fn view(&self) -> Element {
        Flex::column()
            .gap(1)
            .child(
                Flex::row()
                    .border(BorderStyle::Rounded)
                    .padding(1)
                    .child(Text::new(format!("agent: {} lines", self.lines.len())).bold())
                    .child(Spacer::grow())
                    .child(
                        Text::new(if self.running {
                            "● running"
                        } else {
                            "○ idle"
                        })
                        .color(if self.running {
                            ratatui::style::Color::Green
                        } else {
                            ratatui::style::Color::Gray
                        }),
                    ),
            )
            .child(
                ScrollView::new()
                    .flex_grow(1.0)
                    .children(self.lines.iter().map(|l| Text::new(l).into_element())),
            )
            .into_element()
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent, cx: &mut Context<Act>) -> bool {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char('r') => {
                self.running = true;
                let tx = cx.dispatcher();
                cx.spawn(async move {
                    let tools = [
                        "read src/main.rs",
                        "write sprite.png",
                        "grep TODO",
                        "bash cargo test",
                        "read README.md",
                        "write src/lib.rs",
                    ];
                    for (i, t) in tools.iter().enumerate() {
                        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                        tx.dispatch(Act::Line(format!("tool {t} -> ok{i}")));
                    }
                    tx.dispatch(Act::Done);
                });
                true
            }
            _ => false,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = VeloceApp::new();
    app.route("/", AgentDash::default());
    app.run().await
}
