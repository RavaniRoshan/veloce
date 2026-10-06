---
title: Navigate between screens
description: Recipe for push, pop, replace, chrome, and modals.
---

# Navigate between screens

**Problem:** move between screens, keep shared chrome, and open dialogs.

**Steps:**

1. Register every screen once:

```rust
VeloceApp::new()
    .route("/", Dashboard::default())
    .route("/settings", SettingsForm::default())
    .route("/confirm", ConfirmDialog::default())
```

2. Navigate from any view: `cx.navigate("/settings")` pushes,
   `cx.back()` pops, `cx.replace("/")` swaps the top. The root cannot be
   popped — popping it is a no-op that returns `false`.

3. For persistent header/footer chrome, set a `LayoutWrapper` once. It
   survives every push and replace while bodies swap underneath.

4. For a dialog, push the route as a modal. It renders centered over the
   previous route with a dimmed backdrop, and input focus locks to it: keys
   reach the topmost modal first and only fall through unconsumed.

**Variants:** confirm/cancel dialogs are just modal routes whose actions call
`cx.back()` on either choice.
