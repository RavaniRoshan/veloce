---
title: Router reference
description: Route table, stack ops, chrome, and modal semantics.
---

# Router reference

## Registration

```rust
VeloceApp::new()
    .route("/", Dashboard::default())
    .root("/")
```

`route` stores a constructor (`V: View + Clone`); each push clones a fresh
view. `root` resets the stack with an unpoppable root.

## Stack operations

| Operation | Semantics |
|---|---|
| `push(path)` | Construct the view unloaded; it hydrates via `load()`. |
| `pop()` | Remove the top; returns `false` and keeps the root when only one remains. Aborts in-flight loads. |
| `replace(path)` | Pop-then-push atomically; aborts the old load. |
| `push_modal(path)` | Push with overlay rendering and input focus lock. |

`NavCommand::{Push, Pop, Replace}` is what `cx.navigate/back/replace` enqueues;
the router drains the queue once per frame.

## Chrome and overlays

`LayoutWrapper::with_top(...)` / `.with_bottom(...)` renders persistent chrome
around the active route across all navigation. A topmost modal composes as
`Overlay { below: previous route, overlay: modal }`: dimmed backdrop, keys to
the modal first.
