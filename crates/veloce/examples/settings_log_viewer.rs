//! End-to-end: settings form (TextInputs) + streaming log viewer (ScrollView)
//! built only from framework primitives.
use anyhow::Result;
use veloce::prelude::*;

#[derive(Default, Clone)]
struct Dashboard {
    name: TextInput,
    host: TextInput,
    focus: usize,
    logs: Vec<String>,
    offset: u16,
}

#[derive(Clone)]
enum Act {
    Key(crossterm::event::KeyEvent),
}

#[async_trait::async_trait]
impl View for Dashboard {
    type Action = Act;

    async fn load(&mut self) -> Result<()> {
        self.logs.push("dashboard ready".into());
        self.logs.push("watching /var/log/veloce.log".into());
        Ok(())
    }

    fn update(&mut self, action: Act, _cx: &mut Context<Act>) {
        match action {
            Act::Key(k) => match k.code {
                crossterm::event::KeyCode::Tab => {
                    self.focus = (self.focus + 1) % 2;
                }
                crossterm::event::KeyCode::Up => self.offset = self.offset.saturating_sub(1),
                crossterm::event::KeyCode::Down => self.offset = self.offset.saturating_add(1),
                _ => {
                    if self.focus == 0 {
                        self.name.handle_key(&k);
                    } else {
                        self.host.handle_key(&k);
                    }
                }
            },
            
        }
    }

    fn view(&self) -> Element {
        Flex::column()
            .gap(1)
            .child(
                Flex::row()
                    .border(BorderStyle::Rounded)
                    .padding(1)
                    .child(Text::new(" Veloce Settings ").bold()),
            )
            .child(
                Flex::column()
                    .gap(1)
                    .child(Text::new("name:").into_element())
                    .child(self.name.clone().into_element())
                    .child(Text::new("host:").into_element())
                    .child(self.host.clone().into_element()),
            )
            .child(
                ScrollView::new()
                    .offset(self.offset)
                    .flex_grow(1.0)
                    .children(self.logs.iter().map(|l| Text::new(l).into_element())),
            )
            .into_element()
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent, cx: &mut Context<Act>) -> bool {
        cx.dispatch(Act::Key(key));
        true
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = VeloceApp::new();
    app.route("/", Dashboard::default());
    app.run().await
}
