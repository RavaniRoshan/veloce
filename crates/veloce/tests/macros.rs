use veloce::prelude::*;
use veloce::{flex_column, flex_row};

#[test]
fn macros_equal_builder_api() {
    let builder = Flex::column()
        .child(Text::new("a").bold())
        .child(Text::new("b"))
        .into_element();
    let mac = flex_column!(Text::new("a").bold(), Text::new("b"));
    assert_eq!(builder, mac);

    let row_builder = Flex::row().child(Text::new("x")).into_element();
    let row_mac = flex_row!(Text::new("x"));
    assert_eq!(row_builder, row_mac);
}
