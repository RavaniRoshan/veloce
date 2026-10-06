---
title: Layout
description: Flexbox on a terminal cell grid.
---

# Layout

Nest `Flex` containers; Veloce mirrors them into taffy and snaps the result
to terminal cells. Siblings never overlap, narrow containers never collapse
to zero width, and rounding error does not accumulate with nesting depth.

```rust
Flex::column()
    .gap(1)
    .child(
        Flex::row()
            .padding(1)
            .border(BorderStyle::Rounded)
            .child(Text::new("Service: auth-service-v2").bold())
            .child(Spacer::grow())
            .child(Text::new("Status: Live").color(Color::Green)),
    )
    .child(
        ScrollView::new()
            .flex_grow(1.0)
            .children(self.logs.iter().map(Text::new)),
    )
    .into_element()
```

Useful builders: `.gap(n)`, `.padding(n)`, `.border(BorderStyle::Rounded)`,
`.flex_grow(x)`, `.width(SizeSpec::Fixed(n))`, `.child(...)`, `.children(...)`.

The `flex_row!` and `flex_column!` macros expand to exactly the builder tree:

```rust
flex_column![
    Text::new("a").bold(),
    Text::new("b"),
]
```

`Text` wraps to its cell width and honors `.bold()` and `.color()`.
