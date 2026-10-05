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
