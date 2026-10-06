---
title: Views and actions
description: State machines, the Context effect sink, and background tasks.
---

# Views and actions

A view owns its state and reacts to a typed `Action` enum. There is exactly
one place effects happen: the `Context` handed to `update` and `handle_key`.

## The Action loop

```rust
fn update(&mut self, action: Action, cx: &mut Context<Action>) {
    match action {
        Action::DeployRequested => {
            self.status = DeploymentStatus::Deploying;
            let tx = cx.dispatcher(); // clone into background tasks
            cx.spawn(async move {
                for line in stream_logs().await {
                    tx.dispatch(Action::LogReceived(line));
                }
                tx.dispatch(Action::DeployComplete);
            });
        }
        Action::LogReceived(line) => self.logs.push(line),
        Action::DeployComplete => self.status = DeploymentStatus::Live,
    }
}
```

`Context` exposes `dispatch`, `dispatcher`, `spawn`, `navigate`, `back`, and
`replace`. The Tokio handle and router internals stay private — user code
never names them, and no `Arc<Mutex<T>>` appears anywhere in user space.

## Async hydration

Override `load` to fetch what the screen needs. While it runs, `fallback`
renders — by default a loading line, or your own element:

```rust
async fn load(&mut self) -> anyhow::Result<()> {
    self.service = fetch_service().await?;
    Ok(())
}

fn fallback(&self) -> Element {
    Text::new("Loading service…").into_element()
}
```

Navigating away mid-load cancels the task; nothing dangles and nothing
dispatches into a dead view.
