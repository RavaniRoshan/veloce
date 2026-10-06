---
title: Stream logs into a ScrollView
description: Recipe for offset slicing and scrollbars.
---

# Stream logs into a ScrollView

**Problem:** show an unbounded line stream in a bounded viewport.

**Steps:**

1. Keep `logs: Vec<String>` and `offset: u16` in view state. Render:

```rust
ScrollView::new()
    .offset(self.offset)
    .flex_grow(1.0)
    .children(self.logs.iter().map(Text::new))
```

2. Append lines from background actions; leave `offset` alone to pin the
   top, or clamp it to the bottom to tail the stream.
3. Drive `offset` from `Up`/`Down` keys with `saturating_sub`/`saturating_add`
   — the position stays stable across frames.
4. A scrollbar (`█`/`│`) appears automatically once content overflows the
   viewport.

**Check:** push 2,000 lines and confirm the frame stays interactive; the
runnable `agent_dashboard` example streams mock tool output this way:

```bash
cargo run -p veloce --example agent_dashboard
```
