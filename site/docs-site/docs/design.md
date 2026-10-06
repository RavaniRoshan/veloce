---
title: Design
description: Contract, AST shape, quantization engine, and threading topology.
---

# Veloce Design Note (Phase 0)

Status: PROPOSAL. Every checklist row is UNVERIFIED until a named test or probe passes.
Note: `docs/PRD.md` was absent; the task brief in the kickoff message is treated as the PRD text.

## 1. Crate graph (Cargo workspace at repo root)

- `veloce` — facade: re-exports, `prelude::*`, `VeloceApp` (builder + `run()`).
- `veloce-core` — `View`, `AnyView`, `Context<A>`, `Dispatcher<A>`, `Action` types, `Element` AST, `Style`, theme. No terminal I/O.
- `veloce-layout` — taffy mirroring visitor, `quantize_rect`, grid quantization engine. Pure functions of (Element, size).
- `veloce-render` — ratatui buffer rendering; primitive rendering (Text, TextInput, ScrollView, Modal). Tested via TestBackend.
- `veloce-router` — `Router`, `ActiveRoute`, `RouterCommand`, `LayoutWrapper`, modal overlay + focus lock.
- `veloce-runtime` — `TerminalGuard`, panic hook, signal trapping, Tier-1 ingestion thread, dispatcher priority drain, 60 FPS ticker, log-to-file guard.
- `examples/` — reference_app (M4 gate), showcase (M6).

## 2. RESOLVED update/Command contract (needs your decision)

Single model, one signature, one effect sink:

```rust
fn update(&mut self, action: Self::Action, cx: &mut Context<Self::Action>);
```

- `Command` return is removed; tenet 1 is edited to match this signature. A view's effects are exactly: mutate self, call `cx.*`.
- `Context<A>` exposes: `dispatch(action: A)` (enqueue action back to the active view), `dispatcher(&self) -> Dispatcher<A>` (cloneable handle for background tasks; `Dispatcher::dispatch(A)`), `spawn(future)`, `navigate(path)`, `back()`, `key_consumer` helpers.
- The reference example keeps `cx.dispatcher()` as the accessor; `Context::dispatch()` is the same sink for convenience. Both route to the same bounded channel.
- `Context::spawn` is called on the sync UI thread but holds a `tokio::runtime::Handle` captured at runtime construction by `VeloceApp`; user code never sees `Runtime` or `Handle` in public type signatures.

## 3. Element AST + taffy mirroring

`Element` is a tree: `Flex { style, children }`, `Text`, `TextInput`, `ScrollView`, `Modal`, `Spacer`. Mirror pass walks the tree DFS and builds a `TaffyTree` with one node per Element, copying style (display, flex_direction, grow/shrink/basis, gap, padding, border, align/justify). Root computed with definite size from terminal cols/rows. Layout results stored parallel to the tree; render consumes (Element, Rect) pairs. Total by construction.

## 4. Quantization (no drift at depth N)

- Quantize *absolute* edges: `x = round(abs_x)`, `w = round(abs_x + w) - round(abs_x)`. Each edge rounded once — siblings coincide (no gaps/overlaps), decisions independent of depth, so no error accumulation.
- Clamp each rect inside parent's clamped rect first; then if taffy size > 0 and rounding gave 0, force min 1 (parent clamp wins on collision).
- Entry point: `quantize_rect(layout, parent_rect) -> ratatui::Rect`.

## 5. Threading topology

Tier 1: OS thread, blocking `crossterm::event::poll(5ms)`, maps to `AppEvent`, sends on bounded crossbeam channel to Tier 3 (brief block on full = backpressure; resize coalesced; input never dropped).
Tier 2: one `tokio::runtime::Runtime` owned by `VeloceApp`; loaders and `cx.spawn` futures run here; results return only as typed actions.
Tier 3: main thread, synchronous: drain input > actions > tick, update view, render pure (state, size), flush <=60 FPS or dirty flag. No I/O here.

## 6. Conformance matrix (M0-M2, first pass)

A1-A7, B1-B7, C1-C6: all UNVERIFIED. See docs/CHECKLIST.md once created at M0.

## 7. Need from you

- Approve/reject Section-2 contract (required DECISION before View trait).
- Confirm dev-deps: insta, proptest, assert_cmd, expectrl, criterion.
- GitHub target: new remote repo under RavaniRoshan named `veloce`?
