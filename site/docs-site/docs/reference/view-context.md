---
title: View and Context reference
description: Exact methods and contracts, no narrative.
---

# View and Context reference

## `View` trait

`View: Send + Sync + 'static`, with `type Action: Send + Sync + Clone + 'static`.

| Method | Signature | Contract |
|---|---|---|
| `load` | `async fn load(&mut self) -> anyhow::Result<()>` | Runs on the Tokio pool at route entry. Default: `Ok(())`. |
| `fallback` | `fn fallback(&self) -> Element` | Rendered while `load` is pending. Default: loading text. |
| `update` | `fn update(&mut self, action: Self::Action, cx: &mut Context<Self::Action>)` | Pure state transition plus `cx` effects. |
| `view` | `fn view(&self) -> Element` | Declarative tree for the hydrated state. |
| `handle_key` | `fn handle_key(&mut self, key: KeyEvent, cx: &mut Context<Self::Action>) -> bool` | Runs before global routing. `true` consumes. Default: `false`. |

Implementations require `#[async_trait::async_trait]` so `load` futures are `Send`.

## `Context<A>` methods

| Method | Effect |
|---|---|
| `dispatch(action)` | Enqueue an action into this view. |
| `dispatcher()` | Cloneable `Dispatcher<A>` for background tasks. Same sink. |
| `spawn(future)` | Fire-and-forget task on the owned Tokio runtime. |
| `navigate(path)` | Push a route. |
| `back()` | Pop a route. |
| `replace(path)` | Swap the top route. |

The Tokio handle and router internals are private fields; they never appear
in user-facing signatures. Aborting a route's load (pop/replace) drops the
view and silences its dispatchers.
