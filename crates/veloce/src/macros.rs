/// Declarative shorthand equivalent to `Flex::column().child(a).child(b)`.
#[macro_export]
macro_rules! flex_column {
    ($($child:expr),* $(,)?) => {
        $crate::prelude::Flex::column()$(.child($child))*.into_element()
    };
}

/// Declarative shorthand equivalent to `Flex::row().child(a).child(b)`.
#[macro_export]
macro_rules! flex_row {
    ($($child:expr),* $(,)?) => {
        $crate::prelude::Flex::row()$(.child($child))*.into_element()
    };
}
