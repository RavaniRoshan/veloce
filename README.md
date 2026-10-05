# Veloce

Veloce is a Rust terminal-UI meta-framework that brings the Next.js/Remix
developer experience to native terminals: a stack router, declarative views,
taffy-backed flexbox layout, and a frame-budgeted flush — all without manual
event-loop orchestration or borrow-checker contention.

Crates:

- `veloce` — facade, prelude, `VeloceApp`, macros, examples
- `veloce-core` — `View`, `AnyView`, `Context`, `Element` builders
- `veloce-layout` — taffy mirroring + quantization engine
- `veloce-render` — ratatui buffer rendering of `Element` trees
- `veloce-router` — stack router, `LayoutWrapper`, modal overlays
- `veloce-runtime` — terminal guard, ingestion thread, dispatcher, ticker

## Quick start

```rust
use veloce::prelude::*;

#[derive(Default, Clone)]
struct MyView;

#[async_trait::async_trait]
impl View for MyView {
    type Action = ();
    async fn load(&mut self) -> anyhow::Result<()> { Ok(()) }
    fn update(&mut self, _a: (), _cx: &mut Context<()>) {}
    fn view(&self) -> Element {
        Text::new("hello veloce").into_element()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    VeloceApp::new().route("/", MyView).run().await
}
```

## Verification gates

`cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
plus PTY integration tests for panic/signal restoration. Full checklist:
`docs/CHECKLIST.md`; design note: `docs/DESIGN.md`; deferred scope:
`docs/PARITY.md`; progress: `docs/PROGRESS.md`.

## Examples

- `crates/veloce/examples/reference_app.rs` — the reference API in action
- `crates/veloce/examples/settings_log_viewer.rs` — form + log viewer
- `crates/veloce/examples/agent_dashboard.rs` — streaming mock agent output

## Website

`cd site && npm ci && npm run dev` — marketing + docs site (vite).
