---
title: Routing and modals
description: Stack navigation, persistent chrome, and focus-locked overlays.
---

# Routing and modals

Register constructors once, navigate forever:

```rust
VeloceApp::new()
    .route("/", Dashboard::default())
    .route("/settings", SettingsForm::default())
    .route("/confirm", ConfirmDialog::default())
    .run()
    .await
```

From any view: `cx.navigate("/settings")`, `cx.back()`, `cx.replace("/")`.
The first route on the stack is the root and cannot be popped.

## Persistent chrome

Wrap every route in shared header/footer chrome with `LayoutWrapper` — it
survives navigation while route bodies swap underneath.

## Modals

Push a route as a modal to render it centered over the previous route with a
dimmed backdrop. Input focus locks to the modal: keys reach the topmost modal
first, and only unconsumed events fall through.
