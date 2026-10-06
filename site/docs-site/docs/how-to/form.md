---
title: Build a form
description: Recipe for TextInput fields with Tab focus.
---

# Build a form

**Problem:** collect a few text values with visible focus and keyboard-only
operation.

**Steps:**

1. Own one `TextInput` per field plus a `focus: usize` in view state.
2. Render each field from its input: `self.name.clone().into_element()`.
   Empty inputs render their placeholder dimmed.
3. In `handle_key`, advance focus on `Tab` and forward everything else to the
   focused input (see [Handle keys and move focus](/how-to/keys-focus)).
4. Read `.content` from each input when the user confirms; clear or keep the
   values by mutating your own state — the inputs are plain structs.

**Check:** type in field one, press `Tab`, type in field two. Both values
persist and the cursor highlight follows focus. The runnable `settings_log_viewer`
example demonstrates exactly this form next to a live log view:

```bash
cargo run -p veloce --example settings_log_viewer
```
