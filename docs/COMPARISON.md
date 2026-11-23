# Veloce compared with other Rust TUI frameworks

This page is deliberately honest. If another crate fits your app better, use it.
Veloce is an application layer above a renderer, not a replacement for one.

## Quick guide

| If you need… | Reach for |
|---|---|
| Maximum control over rendering, the event loop, and state | [ratatui](https://ratatui.rs/) |
| React-style components, hooks, routing, theming over ratatui | [ratatui-kit](https://crates.io/crates/ratatui-kit) |
| A 5-line immediate-mode app with flexbox and a tiny dependency tree | [SuperLightTUI](https://github.com/subinium/SuperLightTUI) |
| An Elm-style component toolkit | [tuirealm](https://crates.io/crates/tuirealm) |
| A mature widget toolkit with presets | [cursive](https://crates.io/crates/cursive) |
| Multi-screen apps with routing, cancellable async loading, and terminal safety | **Veloce** |

## What each project actually does

| Project | Model | Notable strength | Trade-off |
|---|---|---|---|
| ratatui | Retained widgets, you own the loop | 23.3k GitHub stars, 7,000+ dependent crates, 72.6M downloads; the ecosystem default | The FAQ is explicit: you handle the event loop, app state, and full redraw |
| ratatui-kit | Components, props, hooks, routes over ratatui + Tokio | Component lifetimes, retained state, input layers, theming, playable terminal recordings | A larger abstraction to learn if you only need a single screen |
| SuperLightTUI | Immediate mode, one closure per frame | Flexbox, Tailwind-style chaining, mouse by default, 4 direct dependencies; published benchmark harness | Immediate mode means you redraw every frame and keep state yourself |
| r3bl_tui | Component-based productivity apps | Designed for developer-productivity applications | Smaller ecosystem |
| cursive / tuirealm | Widget and component toolkits | Mature APIs, lots of built-in widgets | Different mental model from route-driven apps |
| Veloce | Pure `View` state machines behind a stack router | Taffy flexbox snapped to cells, `load()` hydration with cancellation, panic/signal terminal recovery, layout cache, live frame stats | Young project; smaller widget catalogue than ratatui |

## Where Veloce wins

- **Routing is part of the framework.** Push, pop, replace, persistent layout chrome, and modal overlays with focus lock come with the router.
- **Async loading has a lifecycle.** `load()` runs on Tokio, `fallback()` renders while pending, and navigation cancels the dangling task. No hook lifetimes or shared locks in user code.
- **Layout is cached.** Repeating a frame with an unchanged element tree and terminal size skips taffy entirely: a 2,000-child frame drops from 2.25 ms cold to 160 µs cached.
- **The terminal is guarded.** Panics, `SIGINT`, and `SIGTERM` restore raw mode, the alternate screen, and mouse capture before exit. Four real-PTY tests cover this.
- **Performance is inspectable.** `frame_stats()` reports measured frame time, and the layout cache can be toggled at runtime.

## Where Veloce does not win

- You need a single screen and want the smallest possible dependency tree — SuperLightTUI is a better fit.
- You want the largest widget catalogue and community — ratatui is the default for a reason.
- You prefer React-style hooks with theming and input layers — ratatui-kit already does that.
- You need mouse-driven interactions today. Veloce tracks keyboard input and restores mouse capture, but click/hover widgets are still on the roadmap.

## Verify any claim yourself

```bash
cargo test --workspace
cargo bench -p veloce --bench frame
cargo run -p veloce-tui --example showcase   # c toggles the cache, v panics
```

Measured numbers in this document come from the benchmark harness in
[`docs/reference/benchmarks.md`](../docs/reference/benchmarks.md) and the
README's verification section.
