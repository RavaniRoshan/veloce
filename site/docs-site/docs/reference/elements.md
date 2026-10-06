---
title: Elements reference
description: Every builder, prop, and style field.
---

# Elements reference

## Containers

`Flex::row()` / `Flex::column()`, plus:

| Builder | Effect |
|---|---|
| `.gap(n)` | Cell gap between children. |
| `.padding(n)` | Uniform inner padding in cells. |
| `.border(BorderStyle::Rounded)` | Border; `Rounded` or `Square`. |
| `.flex_grow(x)` | Share of free space. |
| `.width(SizeSpec::Fixed(n))` / `.height(...)` | Fixed cells; default fills. |
| `.child(...)` / `.children(...)` | Accept anything `Into<Element>`. |
| `.into_element()` | Finish the tree. |

`flex_row![...]` / `flex_column![...]` expand to exactly the builder tree.

## Leaves

| Element | Builders | Notes |
|---|---|---|
| `Text::new(s)` | `.bold()`, `.color(Color)` | Wraps to cell width via layout measure. |
| `TextInput` | `::new()`, `::with_text(s)`, `.placeholder(s)` | Own in state; `.handle_key(&key)` returns consumed. Cursor is char-indexed. |
| `Spacer::grow()` | — | Absorbs free space. |
| `ScrollView::new()` | `.flex_grow(x)`, `.offset(n)`, `.children(...)` | Slices outside the viewport; scrollbar on overflow. |
| `Modal::new(content)` | — | Centered overlay with dimmed backdrop. |
| `Overlay` | `::new(below, overlay)` | Explicit two-layer compose; what modals use. |
