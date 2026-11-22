# Changelog

All notable changes to Veloce are documented here. This project adheres to
[Semantic Versioning](https://semver.org).

## [Unreleased]

### Added — `veloce-core` dispatcher ergonomics (2026-11)

- `Dispatcher::try_dispatch_or_drop(action) -> bool` — non-blocking dispatch
  for lossy, high-frequency events (hover moves, tick heartbeats) where
  backpressuring the producer thread is worse than dropping a stale event.
- `Dispatcher::is_disconnected() -> bool` — lets long-running background
  tasks early-exit once the UI-thread receiver is gone, instead of doing
  work nobody will observe. Implemented with an empty-payload probe so a
  merely-full channel is not misreported as disconnected.
- Unit tests covering delivery, drop-on-full, and disconnect detection
  (`context::dispatcher_tests`).

### Changed

- `.gitignore`: ignore Cargo build artifacts (`/target`) and common editor
  / OS noise so local builds don't pollute `git status`.
