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
- B1..B7: UNVERIFIED (M1 not started).

## C Concurrency and dispatch (M2)
- C1 P0 [T] PARTIAL: channel mapping `ingestion_channel_is_bounded` WORKS; live Tier-1 thread e2e deferred to M2 (pty send-key test).
- C2..C6: UNVERIFIED.

## D..H: UNVERIFIED (later milestones).
