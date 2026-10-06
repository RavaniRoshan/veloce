<div align="center">

```
__     __  _____  _       ___    ____  _____ 
\ \   / / | ____|| |     / _ \  / ___|| ____|
 \ \ / /  |  _|  | |    | | | || |    |  _|  
  \ V /   | |___ | |___ | |_| || |___ | |___ 
   \_/    |_____||_____| \___/  \____||_____|
```

**The terminal-UI meta-framework for Rust — ship terminal apps like web apps.**

[![GitHub stars](https://img.shields.io/github/stars/RavaniRoshan/veloce?style=flat-square)](https://github.com/RavaniRoshan/veloce)
[![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Vite site](https://img.shields.io/badge/site-vite-646CFF?style=flat-square&logo=vite)](./site)

[Features](#features) • [Quick start](#quick-start) • [Usage](#usage) • [Examples](#examples) • [Architecture](#architecture) • [Verification](#verification) • [Docs & site](#docs--site)

</div>

Veloce is an opinionated application-layer framework for terminal UIs. Low-level
crates give you canvas-drawing primitives; Veloce gives you the application:
a **stack router**, **declarative views** with async hydration, **true flexbox
layout** on a discrete cell grid, and a **frame-budgeted render loop** — with
no manual event-loop orchestration and no shared-mutable state in user code.

If you know Next.js or Remix, you already know Veloce: routes render views,
views declare data dependencies, and the framework handles the rest.

> [!NOTE]
> `docs/PRD.md` was never committed to this repo, so the kickoff brief is
> treated as the spec. Every claim in this README is backed by a named test
> or a measured probe — see [Verification](#verification) and
> [`docs/CHECKLIST.md`](./docs/CHECKLIST.md).

## Features

- **Stack router with shells** — push / pop / replace over a route table, an
  unpoppable root, persistent layout chrome (`LayoutWrapper`), and modal
  overlays with backdrop dimming and input focus lock.
- **Elm-inspired state pipeline** — views are pure state machines:
  `update(action, cx)`, no `Arc<Mutex<T>>`, no hook lifetimes. All
  cross-thread communication is typed actions over bounded channels.
- **Managed async hydration** — declare `load()`; the framework runs it on
  Tokio, renders your `fallback()` meanwhile, and cancels dangling tasks when
  you navigate away.
- **True flexbox** — layout is delegated to taffy; a quantization engine snaps
  continuous `f32` layout to terminal cells with error diffusion, parent
  containment clamps, and no zero-width collapses.
- **Crash-resilient runtime** — panics, `SIGINT`, and `SIGTERM` restore the
  terminal (raw mode, alt screen, mouse capture) before exit. Logs go to a
  file, never to the live screen.
- **Built-in primitives** — `Text` (wrapping, bold, color), `TextInput`
  (char-indexed cursor, Unicode-aware), `ScrollView` (offset slicing,
  scrollbar), `Modal`, `Spacer`, plus `flex_row!` / `flex_column!` macros.

## Quick start

**Prerequisites:** a stable Rust toolchain (`rustup default stable`).

Add the facade crate to your `Cargo.toml`:

```toml
[dependencies]
veloce = "0.1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
async-trait = "0.1"
```

Then write a view — one struct, five methods:

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    VeloceApp::new().route("/", Hello).run().await
}
```

```bash
cargo run
```

Press `q` to quit — the terminal is restored on exit. That's the whole app:
no event loop, no channels, no layout math in your code.

> [!TIP]
> Keys reach your view through `handle_key` *before* any global routing.
> Return `true` to consume an event; return `false` and the framework routes
> it onward. `q` is reserved as the global quit key by `VeloceApp::run`.

## Usage

### Views and actions

A view owns its state and reacts to typed actions. Background work feeds back
through a cloneable dispatcher — never a lock:

```rust
fn update(&mut self, action: Action, cx: &mut Context<Action>) {
    match action {
        Action::DeployRequested => {
            self.status = DeploymentStatus::Deploying;
            let tx = cx.dispatcher(); // hand to a background task
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

`Context` is the only effect sink: `dispatch`, `dispatcher`, `spawn`,
`navigate`, `back`, `replace`. The Tokio handle and router internals stay
private — user code never names them.

### Layout

Declare flex containers; the framework mirrors them into taffy and quantizes
the result to cells:

```rust
Flex::column()
    .gap(1)
    .child(
        Flex::row()
            .padding(1)
            .border(BorderStyle::Rounded)
            .child(Text::new("Service: auth-service-v2").bold())
            .child(Spacer::grow())
            .child(Text::new("Status: Live").color(Color::Green)),
    )
    .child(
        ScrollView::new()
            .flex_grow(1.0)
            .children(self.logs.iter().map(Text::new)),
    )
    .into_element()
```

Or use the equivalent macros (`flex_column!` / `flex_row!` expand to exactly
the same tree — proven by `macros_equal_builder_api`):

```rust
flex_column![
    Text::new("a").bold(),
    Text::new("b"),
]
```

### Routing and modals

```rust
VeloceApp::new()
    .route("/", Dashboard::default())
    .route("/settings", SettingsForm::default())
    .route("/confirm", ConfirmDialog::default())
    .run()
    .await
```

From any view: `cx.navigate("/settings")`, `cx.back()`, `cx.replace("/")`.
Push a route with `push_modal` semantics via the router to render it *over*
the previous route with a dimmed backdrop and locked input focus.

## Examples

| Example | What it proves | Run it |
|---|---|---|
| `reference_app` | The reference API end to end: hydration, `d` deploys, streaming logs, `q` quits and restores the terminal | `cargo run -p veloce --example reference_app` |
| `settings_log_viewer` | Settings form (`TextInput` fields, `Tab` focus) + streaming log viewer (`ScrollView`) built only from primitives | `cargo run -p veloce --example settings_log_viewer` |
| `agent_dashboard` | AI coding-agent harness dashboard streaming mock tool executions | `cargo run -p veloce --example agent_dashboard` |
| `panic_view_app` | Panics raised *inside* `view()` (`v`) and *inside* `update()` (`u`) restore the terminal | `cargo run -p veloce --example panic_view_app` |

> [!WARNING]
> The examples take over your terminal (raw mode + alternate screen). If you
> kill one with `SIGKILL`, run `reset` — `SIGINT`/`SIGTERM`/panics are handled,
> `SIGKILL` cannot be.

## Architecture

Three isolated tiers — heavy async work or a resize never drops a frame:

- **Tier 1 · I/O ingestion** — a dedicated OS thread runs blocking
  `crossterm::event::poll(5ms)`, maps events to typed `AppEvent`s, and forwards
  them over a *bounded* crossbeam channel (resize/focus coalesce, keys apply
  backpressure — input is never silently dropped).
- **Tier 2 · Tokio pool** — lifecycle loaders, network/filesystem/process work,
  and `cx.spawn` tasks. Talks to Tier 3 only via typed action dispatch.
- **Tier 3 · UI state machine + renderer** — runs synchronously on the main
  thread: drains input → actions → ticks, updates the active view, and flushes
  at ≤ 60 FPS. No I/O on this thread. Render is a pure function of
  `(state, terminal size)`.

Crate graph:

```
veloce ─┬─ veloce-router ─┬─ veloce-render ─┬─ veloce-layout ── veloce-core
        │                 │                 │                    (View, Context,
        │                 │                 │                     Element, Style)
        │                 │                 └─ ratatui buffer rendering
        │                 └─ stack, LayoutWrapper, modal overlay
        └─ veloce-runtime (TerminalGuard, ingestion, dispatcher, ticker)
```

The layout pipeline is `Element` tree → parallel `taffy::TaffyTree` →
`compute_layout` with definite terminal size → `quantize_rect` (round absolute
edges once, clamp to parent, force min-1 on positive sizes) → `ratatui::Rect`s.
Text nodes are measured with a taffy measure function, so wrapping participates
in layout instead of fighting it.

## Verification

This project follows a simple rule: **a row is WORKS only when an automated
test that drives the real code passes.** Reading code is never proof.

```bash
cargo test --workspace          # 49 tests: unit + insta snapshots + proptest fuzz + real-PTY e2e
cargo fmt --check               # must be clean
cargo clippy --all-targets -- -D warnings   # zero warnings enforced
cargo bench -p veloce --bench frame         # criterion baselines
```

What the suite proves (each row cites its test in
[`docs/CHECKLIST.md`](./docs/CHECKLIST.md)):

- Terminal restored on normal exit, panic, `SIGINT`, `SIGTERM` — on a real PTY.
- Nested flex tiles with no overlaps, no zero-width collapse, no rounding drift
  at depth 12; fuzzed 1×1…500×200 with no panic or overflow.
- Input > actions > ticks priority; 1,000 actions/s leaves key echo at
  p95 ≈ 10 µs; 10,000 interleaved events, zero deadlocks.
- Fallback renders on first frame, hydration completes without flicker,
  mid-load navigation cancels the task; root cannot be popped; modals lock focus.

Measured numbers (Linux x86_64, this repo):

| Budget (spec) | Measured | Status |
|---|---|---|
| Cold boot to first frame < 15 ms | 3.55 ms (criterion) / 6.29 ms (harness) | ✅ |
| Idle memory < 12 MiB | 4 MiB VmRSS | ✅ |
| Key echo under 1k actions/s | p95 10 µs, max 13 µs | ✅ |
| Per-frame cost flat as content grows | 197 µs @10 → 1442 µs @2000 children (linear) | ⚠️ tracked, see below |

> [!NOTE]
> Per-frame cost is *not* flat: relayout is O(children) because the element
> and taffy trees are rebuilt each frame. A memoized layout cache is the
> planned fix — recorded in [`docs/PARITY.md`](./docs/PARITY.md) along with
> everything else deliberately deferred (hot-reload, mouse-wheel scroll,
> non-terminal backends, …).

## Docs & site

- [`docs/DESIGN.md`](./docs/DESIGN.md) — the 70-line design note: contract,
  AST shape, quantization, threading.
- [`docs/CHECKLIST.md`](./docs/CHECKLIST.md) — every requirement with its proof.
- [`docs/PROGRESS.md`](./docs/PROGRESS.md) — milestone log with measured runs.
- [`docs/DECISIONS.md`](./docs/DECISIONS.md) — every judgment call, recorded.
- [`docs/PARITY.md`](./docs/PARITY.md) — capabilities deliberately deferred.
- Marketing + browsable docs site (Vite, inspired by opensend.cc):

```bash
cd site && npm ci && npm run dev     # local preview with hot reload
cd site && npm run build             # static build into site/dist
```
