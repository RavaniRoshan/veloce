use ratatui::layout::Rect;
use veloce_core::{Element, Flex, SizeSpec, Spacer, Text};
use veloce_layout::{quantize_rect, resolve_rects, Placed};

fn assert_siblings_disjoint(placed: &[Placed], parent: usize) {
    let kids: Vec<&Rect> = placed
        .iter()
        .filter(|p| p.parent == parent)
        .map(|p| &p.rect)
        .collect();
    for i in 0..kids.len() {
        for j in (i + 1)..kids.len() {
            let a = kids[i];
            let b = kids[j];
            let overlap_x = a.x < b.x + b.width && b.x < a.x + a.width;
            let overlap_y = a.y < b.y + b.height && b.y < a.y + a.height;
            assert!(
                !(overlap_x && overlap_y),
                "sibling rects overlap: {:?} vs {:?}",
                a,
                b
            );
        }
    }
}

#[test]
fn conversion_is_total() {
    // B1: every AST node has exactly one taffy node.
    let root = Flex::column()
        .child(Text::new("a"))
        .child(Flex::row().child(Text::new("b")).child(Spacer::grow()))
        .into_element();
    let tree = veloce_layout::LayoutTree::build(&root);
    assert_eq!(tree.nodes.len(), 5);
}

#[test]
fn nested_flex_tiles_without_overlap_or_gaps() {
    let root = Flex::column()
        .child(
            Flex::row()
                .padding(1)
                .border(veloce_core::BorderStyle::Rounded)
                .child(Text::new("Service: auth"))
                .child(Spacer::grow())
                .child(Text::new("Status: Ok")),
        )
        .child(
            Flex::column()
                .flex_grow(1.0)
                .child(Text::new("log1"))
                .child(Text::new("log2")),
        )
        .into_element();
    let placed = resolve_rects(&root, 80, 24);
    for p in &placed {
        assert_siblings_disjoint(&placed, p.index);
    }
    // children stay inside parents (B4)
    for p in &placed {
        if p.parent != usize::MAX {
            let parent = placed.iter().find(|q| q.index == p.parent).unwrap();
            assert!(p.rect.x >= parent.rect.x && p.rect.y >= parent.rect.y);
            assert!(p.rect.x + p.rect.width <= parent.rect.x + parent.rect.width);
            assert!(p.rect.y + p.rect.height <= parent.rect.y + parent.rect.height);
        }
    }
}

#[test]
fn quantize_min_one_when_taffy_positive() {
    // B3: taffy assigned 0.4 wide -> rounds to 0 -> force 1.
    let mut layout = taffy::Layout::default();
    layout.size.width = 0.4;
    layout.size.height = 24.0;
    let r = quantize_rect(&layout, Rect::new(0, 0, 10, 24));
    assert_eq!(r.width, 1);
    assert_eq!(r.height, 24);
}

#[test]
fn quantize_clamps_to_parent() {
    // B4
    let mut layout = taffy::Layout::default();
    layout.location.x = 8.0;
    layout.size.width = 10.0;
    let r = quantize_rect(&layout, Rect::new(0, 0, 10, 24));
    assert!(r.x + r.width <= 10);
}

#[test]
fn no_drift_at_depth() {
    // B6: 12-deep nesting, each level border(1)+padding(1): width must shrink
    // by exactly 4 cells per level, never accumulating a rounding error.
    fn build(depth: usize) -> Element {
        if depth == 0 {
            Text::new("x").into_element()
        } else {
            Flex::column()
                .border(veloce_core::BorderStyle::Rounded)
                .padding(1)
                .flex_grow(1.0)
                .height(SizeSpec::Grow(1.0))
                .child(build(depth - 1))
                .into_element()
        }
    }
    let root = Flex::column()
        .width(SizeSpec::Fixed(80))
        .height(SizeSpec::Fixed(24))
        .child(build(12))
        .into_element();
    let placed = resolve_rects(&root, 80, 24);
    // placed[0] = fixed wrapper; placed[1] = build(12) top (full width);
    // each subsequent level must be exactly 4 narrower (border 2 + padding 2).
    let mut expected: Vec<i32> = vec![80];
    for d in 1..12 {
        expected.push(80 - 4 * d);
    }
    let mut chain: Vec<&Placed> = Vec::new();
    let mut cur = placed.iter().find(|p| p.parent == 0).unwrap(); // build top
    chain.push(cur);
    while let Some(child) = placed.iter().find(|p| p.parent == cur.index) {
        chain.push(child);
        cur = child;
    }
    for (i, p) in chain.iter().take(12).enumerate() {
        assert_eq!(
            p.rect.width as i32, expected[i],
            "drift at chain {}: {:?}",
            i, p.rect
        );
    }
    assert_eq!(chain.len(), 13, "12 borders + text leaf");
}

#[test]
fn no_zero_width_collapse() {
    // B3: a growing child must never collapse to 0 in a flex row.
    let root = Flex::row()
        .child(Flex::column().flex_grow(1.0).child(Text::new("body")))
        .child(Text::new("side"))
        .into_element();
    let placed = resolve_rects(&root, 80, 24);
    let kids: Vec<&Placed> = placed.iter().filter(|p| p.parent == 0).collect();
    for k in kids {
        assert!(k.rect.width >= 1 && k.rect.height >= 1);
    }
}
