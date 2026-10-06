---
title: Handle keys and move focus
description: Recipe for handle_key, consuming events, and Tab focus.
---

# Handle keys and move focus

**Problem:** route keystrokes to the right widget and stop them propagating.

**Steps:**

1. Match on `key.code` in `handle_key` and return `true` for every key you
   consume. Unconsumed keys fall through to global routing:

```rust
fn handle_key(&mut self, key: KeyEvent, cx: &mut Context<Action>) -> bool {
    match key.code {
        KeyCode::Char('d') => { cx.dispatch(Action::DeployRequested); true }
        _ => false,
    }
}
```

2. For text fields, own a `TextInput` in view state and forward the key:

```rust
match key.code {
    KeyCode::Tab => { self.focus = (self.focus + 1) % 2; true }
    _ => self.focused_input_mut().handle_key(&key),
}
```

3. `TextInput::handle_key` covers `Char` insert, `Backspace`, arrows, `Home`,
   and `End`. The cursor is char-indexed, so multi-byte input edits correctly.

**Notes:** `q` is reserved as the global quit key by `VeloceApp::run` — don't
bind application behavior to it.
