---
title: Update the UI from background tasks
description: Recipe for cx.spawn and cx.dispatcher().
---

# Update the UI from background tasks

**Problem:** long-running work (log streams, subprocesses, polling) must feed
results into view state without blocking a frame.

**Steps:**

1. In `update`, clone the dispatcher and spawn the work:

```rust
let tx = cx.dispatcher();
cx.spawn(async move {
    while let Some(line) = stream.next().await {
        tx.dispatch(Action::LogReceived(line));
    }
    tx.dispatch(Action::DeployComplete);
});
```

2. Handle each action in `update` as a pure state transition — push the line,
   flip the status. No locks, no channels in your code.

3. The dispatcher is bounded with backpressure: if the UI thread falls
   behind, producers wait instead of dropping input. Size bursts accordingly;
   prefer many small actions over one giant batch.

**Variants:** fire-and-forget notifications need no reply channel — dispatch
and forget. For request/response shapes, dispatch a follow-up action from the
same task.
