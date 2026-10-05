use ratatui::layout::Rect;

/// Single quantization entry point. `layout` is relative to `parent_rect`
/// (taffy computes in continuous f32; terminals are a discrete cell grid).
///
/// Rounding is applied to absolute offsets within the parent, never by
/// truncating the parent's already-quantized rect: a child that fills its
/// parent always inherits the parent's exact size, so error cannot
/// accumulate across nesting depth.
pub fn quantize_rect(layout: &taffy::Layout, parent_rect: Rect) -> Rect {
    let start_x = layout.location.x.round() as i32;
    let start_y = layout.location.y.round() as i32;
    let end_x = (layout.location.x + layout.size.width).round() as i32;
    let end_y = (layout.location.y + layout.size.height).round() as i32;

    let mut x = start_x.clamp(0, parent_rect.width as i32);
    let mut y = start_y.clamp(0, parent_rect.height as i32);

    let mut w = end_x - start_x;
    let mut h = end_y - start_y;

    // Force min 1 when taffy assigned a positive size but rounding zeroed it.
    if layout.size.width > 0.0 && w <= 0 {
        w = 1;
    }
    if layout.size.height > 0.0 && h <= 0 {
        h = 1;
    }
    w = w.max(0);
    h = h.max(0);

    // Parent-containment clamp wins over the min-1 guarantee.
    w = w.min(parent_rect.width as i32 - x).max(0);
    h = h.min(parent_rect.height as i32 - y).max(0);
    x = x.min(parent_rect.width as i32);
    y = y.min(parent_rect.height as i32);

    Rect {
        x: x as u16,
        y: y as u16,
        width: w as u16,
        height: h as u16,
    }
}

/// One placed node: pre-order index, parent's pre-order index
/// (`usize::MAX` for the root), and its absolute quantized rect.
pub struct Placed {
    pub index: usize,
    pub parent: usize,
    pub rect: Rect,
}

/// Walks the tree and returns absolute rects in pre-order, matching
/// `LayoutTree::nodes` indices. `size` is the terminal size in cells.
pub fn resolve_rects(root: &veloce_core::Element, cols: u16, rows: u16) -> Vec<Placed> {
    let mut tree = crate::mirror::LayoutTree::build(root);
    tree.compute(cols, rows);
    let mut out = Vec::new();
    walk(
        root,
        &tree,
        usize::MAX,
        Rect::new(0, 0, cols, rows),
        &mut out,
        &mut 0,
    );
    out
}

fn walk(
    element: &veloce_core::Element,
    tree: &crate::mirror::LayoutTree,
    parent_index: usize,
    parent_abs: Rect,
    out: &mut Vec<Placed>,
    counter: &mut usize,
) {
    let idx = *counter;
    *counter += 1;
    let rel = quantize_rect(tree.layout(idx), parent_abs);
    let abs = Rect {
        x: parent_abs.x + rel.x,
        y: parent_abs.y + rel.y,
        width: rel.width,
        height: rel.height,
    };
    out.push(Placed {
        index: idx,
        parent: parent_index,
        rect: abs,
    });
    let children: &Vec<veloce_core::Element> = match element {
        veloce_core::Element::Flex(f) => &f.children,
        veloce_core::Element::ScrollView(s) => &s.children,
        _ => {
            return;
        }
    };
    for child in children {
        walk(child, tree, idx, abs, out, counter);
    }
}
