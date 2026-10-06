---
title: Fetch data on route entry
description: Recipe for load() and fallback().
---

# Fetch data on route entry

**Problem:** a screen needs remote data before it can render anything useful.

**Steps:**

1. Override `load` and perform the fetch there. It runs on the Tokio pool,
   never on the UI thread:

```rust
async fn load(&mut self) -> anyhow::Result<()> {
    self.service = fetch_service().await?;
    Ok(())
}
```

2. Override `fallback` with what shows while `load` is pending:

```rust
fn fallback(&self) -> Element {
    Text::new("Loading service…").into_element()
}
```

3. Enter the route. The framework renders `fallback` on the first frame and
   swaps in `view()` once `load` resolves — with no flicker in between.

4. If the user navigates away mid-load, the task is cancelled. Nothing
   dispatches into the abandoned view.

**Variants:** return `Err` from `load` to keep showing `fallback`; retry by
re-entering the route. Keep `load` idempotent — entering the same route twice
must be safe.
