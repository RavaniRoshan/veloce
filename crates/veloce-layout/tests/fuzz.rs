//! B7: random container sizes 1x1..500x200 never panic, never overflow,
//! all rects stay in bounds, siblings disjoint.
use proptest::prelude::*;
use veloce_core::{Flex, Spacer, Text};
use veloce_layout::resolve_rects;

fn sample_tree() -> veloce_core::Element {
    Flex::column()
        .child(
            Flex::row()
                .padding(1)
                .border(veloce_core::BorderStyle::Rounded)
                .child(Text::new("Service: {name}"))
                .child(Spacer::grow())
                .child(Text::new("Status")),
        )
        .child(
            Flex::column()
                .flex_grow(1.0)
                .child(Text::new("log a"))
                .child(Text::new("log b")),
        )
        .child(
            Flex::row()
                .gap(2)
                .child(Text::new("x"))
                .child(Text::new("yy")),
        )
        .into_element()
}

proptest! {
    #[test]
    fn fuzz_sizes_never_panic(cols in 1u16..=500, rows in 1u16..=200) {
        let tree = sample_tree();
        let placed = resolve_rects(&tree, cols, rows);
        for p in &placed {
            prop_assert!(p.rect.x <= cols && p.rect.y <= rows, "origin out of bounds: {:?}", p.rect);
            prop_assert!(p.rect.x + p.rect.width <= cols, "overflow x: {:?} in {}x{}", p.rect, cols, rows);
            prop_assert!(p.rect.y + p.rect.height <= rows, "overflow y: {:?} in {}x{}", p.rect, cols, rows);
        }
        // siblings disjoint
        for parent in placed.iter().map(|p| p.index) {
            let kids: Vec<_> = placed.iter().filter(|p| p.parent == parent).collect();
            for i in 0..kids.len() {
                for j in (i + 1)..kids.len() {
                    let (a, b) = (kids[i].rect, kids[j].rect);
                    let ox = a.x < b.x + b.width && b.x < a.x + a.width;
                    let oy = a.y < b.y + b.height && b.y < a.y + a.height;
                    prop_assert!(!(ox && oy), "overlap: {:?} vs {:?}", a, b);
                }
            }
        }
    }
}
