//! A frame is a full redraw. Shrinking or moving text must not leave
//! characters from the previous frame behind.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use veloce_core::{Flex, Text};
use veloce_render::render;

fn symbols(buf: &Buffer) -> String {
    buf.content.iter().map(|cell| cell.symbol()).collect()
}

#[test]
fn shrinking_text_leaves_no_stale_glyphs() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
    render(&Text::new("abcdefghij").into_element(), &mut buf);
    render(&Text::new("ab").into_element(), &mut buf);
    let drawn = symbols(&buf);
    assert!(!drawn.contains("cdef"), "stale glyphs: {drawn:?}");
    assert!(drawn.starts_with("ab "), "new text missing: {drawn:?}");
}

#[test]
fn moving_between_sections_clears_the_old_position() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 30, 4));
    let top = Flex::column()
        .child(Text::new("header"))
        .child(Text::new("first-position"))
        .into_element();
    let bottom = Flex::column()
        .child(Text::new("header"))
        .child(Text::new("second"))
        .into_element();
    render(&top, &mut buf);
    render(&bottom, &mut buf);
    let drawn = symbols(&buf);
    assert!(!drawn.contains("first-position"), "stale row: {drawn:?}");
}
