//! B5: TestBackend string-grid snapshots for every flex combination.
use veloce_core::{BorderStyle, Flex, Spacer, Text};
use veloce_render::render_to_string;

fn normalize(grid: &str) -> String {
    grid.lines()
        .map(|l| l.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn snapshot_row() {
    let el = Flex::row()
        .padding(1)
        .gap(2)
        .child(Text::new("a"))
        .child(Text::new("b"))
        .into_element();
    insta::assert_snapshot!(normalize(&render_to_string(&el, 20, 5)));
}

#[test]
fn snapshot_column() {
    let el = Flex::column()
        .padding(1)
        .child(Text::new("top"))
        .child(Text::new("bottom"))
        .into_element();
    insta::assert_snapshot!(normalize(&render_to_string(&el, 20, 5)));
}

#[test]
fn snapshot_grow_shrink() {
    let el = Flex::row()
        .child(Flex::column().flex_grow(1.0).child(Text::new("grow1")))
        .child(Flex::column().flex_grow(3.0).child(Text::new("grow3")))
        .into_element();
    insta::assert_snapshot!(normalize(&render_to_string(&el, 40, 3)));
}

#[test]
fn snapshot_gap() {
    let el = Flex::row()
        .gap(3)
        .child(Text::new("x"))
        .child(Text::new("y"))
        .child(Text::new("z"))
        .into_element();
    insta::assert_snapshot!(normalize(&render_to_string(&el, 25, 2)));
}

#[test]
fn snapshot_padding_border() {
    let el = Flex::column()
        .border(BorderStyle::Rounded)
        .padding(1)
        .child(Text::new("inner"))
        .into_element();
    insta::assert_snapshot!(normalize(&render_string(&el, 20, 5)));
}

#[test]
fn snapshot_nested_header_body() {
    let el = Flex::column()
        .child(
            Flex::row()
                .padding(1)
                .border(BorderStyle::Rounded)
                .child(Text::new("Service: auth"))
                .child(Spacer::grow())
                .child(Text::new("Status: Ok")),
        )
        .child(
            Flex::column()
                .flex_grow(1.0)
                .child(Text::new("log line 1"))
                .child(Text::new("log line 2")),
        )
        .into_element();
    insta::assert_snapshot!(normalize(&render_to_string(&el, 60, 10)));
}

fn render_string(el: &veloce_core::Element, w: u16, h: u16) -> String {
    render_to_string(el, w, h)
}
