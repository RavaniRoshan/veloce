---
title: Architecture
description: Why three tiers, and why actions instead of locks.
---

# Architecture

Three isolated tiers exist so heavy async work or a resize can never cause a
frame drop or input lag:

- **Tier 1, I/O ingestion.** A dedicated OS thread runs blocking
  `crossterm::event::poll` and forwards typed events over a bounded channel.
  Resize and focus events coalesce; keys apply backpressure. Input is never
  silently dropped.
- **Tier 2, Tokio pool.** Loaders, network, filesystem, and `cx.spawn` tasks.
  It talks to the UI only through typed action dispatch.
- **Tier 3, UI state machine.** Runs synchronously on the main thread: drain
  inputs, then actions, then ticks; update the active view; render as a pure
  function of state and terminal size; flush at most at 60 FPS. No I/O here.

State moves as messages, Elm-style, because shared-mutable state across an
event thread, a task pool, and a render loop is where terminal apps go to
die. Each tier owns its data; the only coupling is typed, bounded channels.
Merging any two tiers would reintroduce exactly the contention the framework
exists to remove: blocked frames during loads, dropped keys during floods,
and teardown races on exit.
