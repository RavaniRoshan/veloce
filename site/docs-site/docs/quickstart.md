---
title: Quickstart
description: Install Veloce and render your first frame.
---

# Quickstart

Requires a stable Rust toolchain.

## 1. Depend

```toml
[dependencies]
veloce = "0.1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
async-trait = "0.1"
```

## 2. Declare a view

```rust
use veloce::prelude::*;

#[derive(Default, Clone)]
struct Hello;

#[derive(Clone)]
enum Action {
    Greeted,
}

#[async_trait::async_trait]
impl View for Hello {
    type Action = Action;

    async fn load(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self, action: Action, cx: &mut Context<Action>) {
        match action {
            Action::Greeted => cx.navigate("/done"),
        }
    }

    fn view(&self) -> Element {
        Flex::column()
            .padding(1)
            .border(BorderStyle::Rounded)
            .child(Text::new("hello veloce").bold())
            .into_element()
    }
}
```

## 3. Run it

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    VeloceApp::new().route("/", Hello).run().await
}
```

```bash
cargo run
```

Press `q` to quit — the terminal is restored on exit.

## Keys

Override `handle_key` to intercept keys before global routing. Return `true`
to consume the event:

```rust
fn handle_key(&mut self, key: KeyEvent, cx: &mut Context<Action>) -> bool {
    match key.code {
        KeyCode::Char('d') => { cx.dispatch(Action::DeployRequested); true }
        _ => false,
    }
}
```

`q` is reserved as the global quit key by `VeloceApp::run`.
