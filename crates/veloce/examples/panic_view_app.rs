//! A2 proof app: panics INSIDE view() on frame 2, and inside update() when
//! the key 'u' is pressed. The framework must restore the terminal in both.
use anyhow::Result;
use veloce::prelude::*;

#[derive(Default, Clone)]
struct Fragile {
    mode: String,
}

#[derive(Clone)]
enum Act {
    Boom,
}

#[async_trait::async_trait]
impl View for Fragile {
    type Action = Act;

    async fn load(&mut self) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, action: Act, _cx: &mut Context<Act>) {
        match action {
            Act::Boom => {
                self.mode = "boom".into();
                panic!("intentional panic inside update()");
            }
        }
    }

    fn view(&self) -> Element {
        if self.mode == "panic-view" {
            panic!("intentional panic inside view()");
        }
        Text::new("stable").into_element()
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent, cx: &mut Context<Act>) -> bool {
        match key.code {
            crossterm::event::KeyCode::Char('v') => {
                self.mode = "panic-view".into();
                true
            }
            crossterm::event::KeyCode::Char('u') => {
                cx.dispatch(Act::Boom);
                true
            }
            _ => false,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = VeloceApp::new();
    app.route("/", Fragile::default());
    app.run().await
}
