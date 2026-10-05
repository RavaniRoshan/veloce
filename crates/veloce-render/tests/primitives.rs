//! F1 Text wrapping+ANSI colour, F2 TextInput, F3 ScrollView, F4 Modal.
use veloce_core::{BorderStyle, Flex, Modal, ScrollView, Text, TextInput};
use veloce_render::render_to_string;

#[test]
fn text_wraps_at_width() {
    let el = Flex::column()
        .width(veloce_core::SizeSpec::Fixed(5))
        .height(veloce_core::SizeSpec::Fixed(4))
        .child(Text::new("hello world veloce"))
        .into_element();
    let grid = render_to_string(&el, 10, 8);
    assert!(grid.contains("hello"), "grid: {grid}");
    assert!(
        grid.lines().nth(1).unwrap().starts_with(" worl"),
        "wrap line2: {grid}"
    );
    assert!(
        grid.lines().nth(2).unwrap().starts_with("d vel"),
        "wrap line3: {grid}"
    );
}

#[test]
fn textinput_insert_backspace_cursor() {
    let mut ti = TextInput::new();
    for c in ['h', 'i'] {
        ti.handle_key(&crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char(c),
            crossterm::event::KeyModifiers::NONE,
        ));
    }
    assert_eq!(ti.content, "hi");
    assert_eq!(ti.cursor, 2);
    ti.handle_key(&crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Backspace,
        crossterm::event::KeyModifiers::NONE,
    ));
    assert_eq!(ti.content, "h");
    assert_eq!(ti.cursor, 1);
    ti.handle_key(&crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Left,
        crossterm::event::KeyModifiers::NONE,
    ));
    assert_eq!(ti.cursor, 0);
    ti.handle_key(&crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('你'),
        crossterm::event::KeyModifiers::NONE,
    ));
    ti.handle_key(&crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('好'),
        crossterm::event::KeyModifiers::NONE,
    ));
    assert_eq!(ti.content, "你好h");
    assert_eq!(ti.cursor, 2);
}

#[test]
fn textinput_renders_cursor() {
    let ti = TextInput::with_text("ab");
    let el = ti.into_element();
    // TextInput is sized to max(content,10) cols by mirror: content 2 -> 10 cols
    let grid = render_to_string(&el, 12, 1);
    assert!(grid.starts_with("ab"), "grid: {grid:?}");
}

#[test]
fn scrollview_slices_and_scrollbar() {
    let sv = ScrollView::new()
        .offset(2)
        .flex_grow(1.0)
        .children((0..8).map(|i| Text::new(format!("line{i}")).into_element()));
    let el = Flex::column().child(sv).into_element();
    let grid = render_to_string(&el, 20, 4);
    assert!(grid.contains("line2"), "offset did not slice: {grid}");
    assert!(
        !grid.contains("line0"),
        "line0 should be scrolled away: {grid}"
    );
    assert!(
        grid.contains('█') || grid.contains('│'),
        "no scrollbar: {grid}"
    );
}

#[test]
fn modal_renders_centered_with_backdrop() {
    let m = Modal::new(
        Flex::column()
            .border(BorderStyle::Rounded)
            .child(Text::new("Confirm?"))
            .into_element(),
    );
    let el = m.into_element();
    let grid = render_to_string(&el, 30, 8);
    assert!(grid.contains("Confirm?"), "grid: {grid}");
    assert!(grid.contains('░'), "no backdrop: {grid}");
}
