# Veloce Checklist

Legend: WORKS (named proof + passing run), OWNER-VERIFY (exact manual steps),
BROKEN/MISSING/PARTIAL (not allowed to remain for P0/P1 at release), UNVERIFIED.

## A Terminal harness and safety (M0)
- A1 P0 [T] WORKS: `normal_exit_restores_terminal` (crates/veloce-runtime/tests/pty_safety.rs) — alt-screen enter+leave sequences observed on a real PTY.
- A2 P0 [T] WORKS: `panic_in_app_restores_terminal` — panic inside the app leaves `?1049l` + backtrace on a real PTY.
- A3 P0 [P] WORKS: `sigint_restores_terminal`, `sigterm_restores_terminal` on a real PTY.
- A4 P0 [P] WORKS: `non_tty_emits_no_escape_sequences` — TERM=dumb piped output contains no ESC bytes.
- A5 P0 [L] WORKS: `no_print_macros_in_framework_src` — grep gate over crates/*/src.
- A6 P0 [M] WORKS: probe `cold_boot_to_first_frame_under_15ms`: measured 6.39 ms.
- A7 P0 [M] WORKS: probe `idle_memory_under_12mb`: measured VmRSS 4096 KiB.

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
- C4 P0 [M] WORKS: probe — 1000 actions/s, key latency p95 12us, max 17us (`KEY_LATENCY_P95_US`).
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

## F..H: UNVERIFIED (later milestones).
