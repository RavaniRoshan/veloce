## Streak days covered
| Date | Commit | What |
|---|---|---|
| Fri 13 Nov 2026 | `78bc6eb` | chore: fill `.gitignore` (Cargo artifacts, editor/OS noise) |
| Sun 15 Nov 2026 | `a917e19` | feat(core): lossy dispatch + disconnect detection on `Dispatcher` |
| Mon 16 Nov 2026 | `0eaa045` | docs: add `CHANGELOG.md` |
| Thu 19 Nov 2026 | `dfe9487` | chore: verification pass note |

## Summary of improvements
- **`.gitignore`**: `/target`, common editor swap/backup files, `.DS_Store`, site `node_modules` — repo hygiene so build junk never lands in commits.
- **`veloce-core::Dispatcher`** (`crates/veloce-core/src/context.rs`, +54 lines):
  - `try_dispatch_or_drop(event)` — non-blocking send that drops the event when the channel is full. Intended for high-frequency events (hover moves, tick heartbeats) where stale events are worse than dropped ones.
  - `is_disconnected()` — empty-payload probe to detect a closed receiver without misreporting a full-but-alive channel as dead.
  - 3 new unit tests covering both behaviors.
- **`CHANGELOG.md`**: records the above under an Unreleased section.

## Test plan
- [x] `cargo test -p veloce-core --lib` — all dispatcher tests pass (Rust 1.100)
- [ ] CI green
- [ ] Confirm merge to default branch so the 4 dates register on the contribution calendar

## Notes
- Each day has exactly one commit with a real diff, which is enough to count per-day on the GitHub contribution graph once merged to the **default branch**.
