---
title: Progress
description: Milestone log with measured runs.
---

# Progress

## 2026-10-05 M0
- Workspace created: veloce facade + veloce-core/layout/render/router/runtime.
- TerminalGuard/panic hook/signals/RAII; PTY-verified panic+SIGINT+SIGTERM+normal restore.
- Tier-1 ingestion thread + bounded crossbeam; 60Hz ticker; log-to-file.
- A5 lint gate; A4 non-TTY no-escapes; A6 6.39ms; A7 4MiB.
- cargo fmt clean; clippy: zero warnings at M2 end.
- M1 next: Element AST, taffy mirror, quantize_rect + drift/fuzz/snapshot tests.

## 2026-10-05 M1
- Element AST + builders (Flex row/column/gap/padding/border/grow/width/height, Text, Spacer, ScrollView).
- LayoutTree DFS mirror into TaffyTree; compute with definite terminal size.
- quantize_rect rounds absolute edges; min-1 on positive taffy size; parent clamp wins.
- resolve_rects -> Placed (index,parent,rect); render(&Element)->Buffer pure.
- B1-B7 all WORKS with tests: layout_invariants.rs, fuzz.rs, snapshots.rs (insta).
- Auto-size containers default to fill parent (Dimension::percent(1.0)).

## 2026-10-05 M2
- Context<A>/Dispatcher<A>/NavCommand in veloce-core; spawn via private Handle.
- drain_prioritized(budget): inputs > actions > ticks; C3/C4/C5 probes pass.
- C4 key latency p95 12us under 1000 actions/s; C5 10k interleaved, zero deadlock.
- C1 PTY e2e ingest verified; C6 shared-mut lint gate added.

Milestones 0-2 complete. All P0 rows in A, B, C are WORKS or OWNER-VERIFY; zero BROKEN/MISSING/PARTIAL.

## 2026-10-05 __M3-M4__
- View trait (async_trait load/fallback/update/view/handle_key), ViewAdapter with hydration machine (Idle->Loading->Ready|Failed), abort_load.
- AnyView dyn dispatch + ViewAdapter; D1-D4 tests pass.
- Element::Overlay centered 60% modal + darkened backdrop via taffy Position::Absolute; Placed rects via DFS walk.
- Router: route table, stack, push/pop/replace, root-pop refused, modal key precedence, NavCommand queue via cx.
- VeloceApp::run: ingestion thread, 60 FPS interval, router.step, CrosstermBackend draw, 'q' quits.
- reference_app example runs on PTY: hydration, deploy streaming, Live, terminal restored (output evidenced).

## 2026-10-05 M5
- Text: measured by taffy measure function -> wrap-aware layout; bold/color preserved.
- TextInput: char-indexed cursor, insert/backspace/arrows/home/end, placeholder; U+4F60U+597D handled via char-indexed cursor.
- ScrollView: offset slicing, viewport clipping, █/│ scrollbar; stable position.
- Modal: Element::Modal renders centered, dimmed backdrop.
- B7 overlap found & fixed via taffy measure driving Text layout; regression absent.
- F5: settings_log_viewer example + PTY test passes.

## 2026-10-05 M6(partial dev)
- flex_column!/flex_row! macros == builder (G1).
- criterion benches: cold_boot 3.55ms, frame cost linear ~0.7us/child (G2 partial).
- agent_dashboard showcase + PTY test (G3).

## 2026-10-05 M6 completion + release prep
- A2 hardened: panic inside view() and inside update() of a live VeloceApp proven on PTY (panic_in_view_pty.rs, 2 tests).
- README.md added; site/ (vite marketing + docs pages from docs/*.md) builds (npx vite build OK).
- Final gates: cargo fmt --check clean; cargo clippy --all-targets -- -D warnings clean; cargo test --workspace 48 tests green; no #[ignore] anywhere in crates/.
- Measured this run: cold boot 6.29 ms (harness) / 3.55 ms (criterion), idle RSS 4 MiB, key latency p95 10 us.
- G4/G5 remain OWNER-VERIFY (no macOS/Windows toolchain; no crates.io token).

## 2026-10-05 Ship: GitHub + site
- Repo: https://github.com/RavaniRoshan/veloce (public, branch master @ 21e00ce; force-pushed once to purge accidentally committed target/ + rebuild artifacts from history; .git 296K; `git status` clean; remote HEAD == local HEAD 21e00ce verified via ls-remote).
- Caught and fixed before ship: first push contained target/debug binaries committed before .gitignore existed (16,828 tracked build files). History rewritten with git filter-branch to drop target/site node_modules, gc'd aggressively, force-pushed.
- Final gates on the shipped commit: cargo fmt --check clean; cargo clippy --all-targets -- -D warnings clean; cargo test --workspace: 49 passed, 0 failed.
- Site: site/ builds (`npm run build` OK, 8 modules) and serves: index + docs.html + /docs/*.md all verified over vite preview (HTTP 200).
