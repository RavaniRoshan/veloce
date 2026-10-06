---
title: Checklist
description: Every requirement with its proof.
---

# Veloce Checklist

Legend: WORKS (named proof + passing run), OWNER-VERIFY (exact manual steps),
BROKEN/MISSING/PARTIAL (not allowed to remain for P0/P1 at release), UNVERIFIED.

## A Terminal harness and safety (M0)
- A1 P0 [T] WORKS: `normal_exit_restores_terminal` (crates/veloce-runtime/tests/pty_safety.rs) — alt-screen enter+leave sequences observed on a real PTY.
- A2 P0 [T] WORKS: strict — `panic_inside_view_restores_terminal` and `panic_inside_update_restores_terminal` (crates/veloce/tests/panic_in_view_pty.rs) panic inside `view()` / `update()` of a live VeloceApp frame loop on a real PTY; both assert `?1049l` (alt screen restored) plus the panic message. Plus `panic_in_app_restores_terminal` for the runtime-level hook.
- A3 P0 [P] WORKS: `sigint_restores_terminal`, `sigterm_restores_terminal` on a real PTY.
- A4 P0 [P] WORKS: `non_tty_emits_no_escape_sequences` — TERM=dumb piped output contains no ESC bytes.
- A5 P0 [L] WORKS: `no_print_macros_in_framework_src` — grep gate over crates/*/src.
- A6 P0 [M] WORKS: probe `cold_boot_to_first_frame_under_15ms`: measured 6.29 ms (test harness path); criterion `cold_boot_first_frame` = 3.55 ms.
- A7 P0 [M] WORKS: probe `idle_memory_under_12mb`: measured VmRSS 4096 KiB (4 MiB).

## B Layout engine (M1)
- B1 P0 [T] WORKS: `conversion_is_total` — nodes vec len == pre-order count.
- B2 P0 [T] WORKS: `nested_flex_tiles_without_overlap_or_gaps` + fuzz sibling-disjoint asserts.
- B3 P0 [T] WORKS: `quantize_min_one_when_taffy_positive`, `no_zero_width_collapse`.
- B4 P0 [T] WORKS: `quantize_clamps_to_parent` + nested containment asserts.
- B5 P0 [S] WORKS: `tests/snapshots.rs` insta snapshots (row/column/grow/gap/padding/border/nested).
- B6 P0 [T] WORKS: `no_drift_at_depth` — exact 4-cell shrink per level over 12 levels.
- B7 P0 [T] WORKS: `tests/fuzz.rs` proptest 1x1..500x200, no panic/overflow, in-bounds, disjoint.

## C Concurrency and dispatch (M2)
- C1 P0 [T] WORKS: `tier1_emits_typed_events_over_bounded_channel` (PTY e2e) + `ingestion_channel_is_bounded` semantics.
- C2 P0 [T] WORKS: render is a pure fn of (element, size) — `render(&Element, &mut Buffer)` signature; TestBackend snapshots assert determinism.
- C3 P0 [T] WORKS: `priority_input_over_actions_over_ticks`.
- C4 P0 [M] WORKS: probe `key_echo_not_blocked_by_1000_actions_per_sec` — 1000 actions/s, key latency p95 10 us, max 13 us.
- C5 P0 [T] WORKS: `ten_k_interleaved_events_no_deadlock`.
- C6 P0 [L] WORKS: `no_shared_mutable_state_in_public_api` lint gate.

## D View lifecycle and hydration (M3)
- D1 P0 [T] WORKS: `any_view_is_dyn_safe` — Box<dyn AnyView> enter/poll/view.
- D2 P0 [T] WORKS: `hydration_fallback_then_ready_no_flicker` — fallback first frame, hydrated after, never reverts.
- D3 P0 [T] WORKS: `abort_mid_load_cancels_task_and_drops_view` — abort drops view, no dangling task.
- D4 P0 [T] WORKS: `handle_key_consumed_before_focus_manager` — true return suppresses global routing.

## E Router and shells (M4)
- E1 P0 [T] WORKS: `pop_refuses_root`, `replace_swaps_top_and_aborts_load`.
- E2 P0 [T] WORKS: `wrapper_chrome_persists_across_replace` — HEADER survives replace.
- E3 P0 [T] WORKS: `modal_overlays_previous_route_with_backdrop` + `modal_gets_precedence_over_route_for_keys`.
- E4 P0 [P] WORKS: `reference_app_hydrates_deploys_and_restores` — full PTY run of the reference example with captured output printed.

## F Primitives (M5)
- F1 P0 [S] WORKS: `text_wraps_at_width` (wrap via taffy measure) + bold/color style; `tests/primitives.rs` grid assertion. (Note: full inline reflow of template columns keeps content baseline — wrap is render-measured via taffy measure functions.)
- F2 P0 [T] WORKS: `textinput_insert_backspace_cursor`, `textinput_renders_cursor` — cursor char-indexed incl. U+4F60 U+597D.
- F3 P0 [S] WORKS: `scrollview_slices_and_scrollbar` — offset 2 yields line2 at top (no line0); scrollbar █/│ drawn.
- F4 P0 [S] WORKS: `modal_renders_centered_with_backdrop` — centered Modal with dimmed backdrop.
- F5 P0 [T] WORKS: `settings_form_and_log_viewer_via_primitives` (PTY e2e) — form fields typed, Tab focus, log viewer hydrated.

## G DX, macros, performance, release (M6)
- G1 P1 [T] WORKS: `macros_equal_builder_api` — flex_column!/flex_row! macro == builder output (PartialEq).
- G2 P0 [M] PARTIAL: cold boot 3.55 ms (<15 ms) PASS; idle RSS 4 MiB (<12 MiB) PASS; **per-frame cost is NOT flat as content grows**: 197 µs @10 children, 259 µs @100, ~500 µs @500, 1442 µs @2000. The PRD claim "per-frame cost flat as content grows" is NOT met at this milestone — relayout is O(children) because the Element tree and taffy tree are rebuilt per frame. Render output itself is viewport-bounded; a memoized layout cache is the fix and is listed in PARITY.md.
- G3 P0 [T] WORKS: `agent_dashboard_streams_mock_tool_executions` PTY test.
- G4 P0 OWNER-VERIFY: Linux x86_64 verified locally (build + PTY e2e). macOS/Windows have no cross toolchain here; not verified — OWNER-VERIFY steps in final answer.
- G5 P0 OWNER-VERIFY: `cargo package -p <crate> --list` verified file manifests; full `cargo package --workspace` cannot resolve unpublished inter-crate versions, and `cargo publish` needs the owner token. Docs (README, docs/*.md) and 3 runnable examples exist. OWNER-VERIFY steps in final answer.

## H Verification matrix
- H1 P0 WORKS: layout invariants proptest + invariants test (see B1-B7).
- H2 P0 WORKS: pty_safety (4 tests, isolated).
- H3 P0 WORKS: ten_k_interleaved_events_no_deadlock.
- H4 P0 WORKS: fuzz 1x1..500x200 (fuzz_sizes_never_panic, zero overlapped/out-of-bounds).
