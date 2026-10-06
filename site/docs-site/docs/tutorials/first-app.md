---
title: Build your first app
description: One guided lesson from install to a running dashboard.
---

# Build your first app

In this lesson you will build and run a small deployment dashboard. Follow
the steps in order; each one ends with something you can check.

## 1. Depend

You need a stable Rust toolchain. Add this to `Cargo.toml`:

```toml
[dependencies]
veloce = "0.1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
async-trait = "0.1"
```

Check: `cargo build` succeeds.

## 2. Declare a view

A view is a struct plus an `Action` enum plus five methods. Create
`src/main.rs`:

```rust
use veloce::prelude::*;

#[derive(Default, Clone)]
struct Dashboard {
    service_name: String,
    status: String,
    logs: Vec<String>,
}

#[derive(Clone)]
enum Action {
    DeployRequested,
    LogReceived(String),
    DeployComplete,
}

#[async_trait::async_trait]
impl View for Dashboard {
    type Action = Action;

    async fn load(&mut self) -> anyhow::Result<()> {
        self.service_name = "auth-service-v2".into();
        self.status = "Idle".into();
        Ok(())
    }

    fn update(&mut self, action: Action, cx: &mut Context<Action>) {
        match action {
            Action::DeployRequested => {
                self.status = "Deploying".into();
                let tx = cx.dispatcher();
                cx.spawn(async move {
                    for i in 0..5 {
                        tx.dispatch(Action::LogReceived(format!("pod log line {i}")));
                    }
                    tx.dispatch(Action::DeployComplete);
                });
            }
            Action::LogReceived(line) => self.logs.push(line),
            Action::DeployComplete => self.status = "Live".into(),
        }
    }

    fn view(&self) -> Element {
        Flex::column()
            .gap(1)
            .child(
                Flex::row()
                    .padding(1)
                    .border(BorderStyle::Rounded)
                    .child(Text::new(format!("Service: {}", self.service_name)).bold())
                    .child(Spacer::grow())
                    .child(Text::new(format!("Status: {}", self.status))),
            )
            .child(
                ScrollView::new()
                    .flex_grow(1.0)
                    .children(self.logs.iter().map(Text::new)),
            )
            .into_element()
    }

    fn handle_key(&mut self, key: KeyEvent, cx: &mut Context<Action>) -> bool {
        match key.code {
            KeyCode::Char('d') => { cx.dispatch(Action::DeployRequested); true }
            _ => false,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    VeloceApp::new().route("/", Dashboard::default()).run().await
}
```

Check: `cargo build` succeeds with no warnings.

## 3. Run it

```bash
cargo run
```

You will see a bordered header with the service name and a log area.
Press `d`: the status flips to `Deploying` and five log lines stream in,
then `Live`. Press `q` to quit — the terminal is restored.

Check: after quitting, your shell prompt behaves normally (type `echo hi`).

## Where next

- Something fails? Read the [FAQ](/explanation/faq).
- Want data from your own source? Follow
  [Fetch data on route entry](/how-to/fetch-data).
- Want a second screen? Follow [Navigate between screens](/how-to/navigate-modal).
