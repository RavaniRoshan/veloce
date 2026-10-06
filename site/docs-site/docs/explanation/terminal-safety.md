---
title: Terminal safety
description: Why restoration is a framework responsibility.
---

# Terminal safety

A terminal app that exits dirty — stuck in raw mode, hidden cursor, stranded
alt screen — destroys trust in one incident. Users cannot be expected to get
teardown ordering right across panics, signals, and normal exits, so Veloce
treats restoration as framework-owned infrastructure:

- A RAII guard enables raw mode, the alternate screen, and mouse capture, and
  reverses all of it on drop.
- A panic hook restores the terminal *before* the default hook prints the
  backtrace.
- `SIGINT`/`SIGTERM` handlers restore the terminal and flush logs before exit.
- Non-TTY or `TERM=dumb` environments emit no escape sequences at all.
- Nothing writes to stdout/stderr while the app is live; logs go to a file.

The only unrecoverable case is `SIGKILL`, which the OS never delivers to
handlers — run `reset` if that happens.
