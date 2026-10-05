# Veloce PARITY — capabilities deliberately deferred

The Rust TUI ecosystem baseline (ratatui + crossterm) provides canvas-drawing
primitives, not application architecture. Veloce aims to give web-developers
Next.js/Remix-style architecture and Rust-developers idiomatic safety. It does
not, by this contract version (0.1.0), include:

- Hot-reload / dev-server: dev experience of iterating without restart (M7 candidate).
- Direct IDE integration / LSP client.
- Mouse-aware scrolling in ScrollView: mouse events decoded but no scroll wheel wiring.
- True subtree offset translation for ScrollView of non-Text children (complex containers render first-frame placeholder).
- List virtualization memory optimization (render cost is linear with children).
- Theme system centralization beyond per-element colors.
- Non-terminal backends (Web, Wasm): out of scope by design.
- Keybinding system / global focus manager modal recording beyond handle_key priority.
- Windows-specific packaging/cross-compile checks (no CI matrix in repo yet).
- Publishing automation (crates.io publish, binary releases): owner-verified only.

## Supported contract

- View: async load/fallback/update/view/handle_key; per-action Dispatcher via Context.
- Element/Flex/Text/Spacer/ScrollView/TextInput/Modal/Overlay trees.
- Router: push/pop/replace stack, root-pop refused, modal overlay + focus precedence.
- 60 FPS frame budget, ingestion thread, tokio runtime owned by VeloceApp::run.
- Panic/SIGINT/SIGTERM restore the terminal (proven by PTY tests).
