---
title: Primitives
description: Text, TextInput, ScrollView, Modal, Spacer.
---

# Primitives

Everything below is built into the framework — compose them, don't reimplement them.

## Text

```rust
Text::new("Status: Live").bold().color(Color::Green)
```

Wraps to its cell width. Empty `TextInput`s render their placeholder dimmed.

## TextInput

Own one in your view state, render a clone, and forward keys:

```rust
struct Form { name: TextInput, focus: usize }

fn view(&self) -> Element {
    self.name.clone().into_element()
}

fn handle_key(&mut self, key: KeyEvent, cx: &mut Context<Action>) -> bool {
    match key.code {
        KeyCode::Tab => { self.focus += 1; true }
        _ => self.name.handle_key(&key),
    }
}
```

The cursor is char-indexed, so multi-byte input edits correctly.
`Backspace`, arrows, `Home`, and `End` all work.

## ScrollView

```rust
ScrollView::new()
    .offset(self.scroll)
    .flex_grow(1.0)
    .children(self.logs.iter().map(Text::new))
```

Children outside the viewport are sliced away; a scrollbar appears when
content overflows. Drive `offset` from `Up`/`Down` keys.

## Modal and Spacer

`Modal::new(content)` centers content with a dimmed backdrop. `Spacer::grow()`
absorbs free space in a row — use it to right-align status text.
