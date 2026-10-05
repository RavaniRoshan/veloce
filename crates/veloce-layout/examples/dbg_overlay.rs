fn main() {
    let el = veloce_core::Element::Overlay {
        below: Box::new(veloce_core::Element::Flex(
            veloce_core::Flex::column().child(veloce_core::Text::new("BELOW")),
        )),
        overlay: Box::new(veloce_core::Element::Flex(
            veloce_core::Flex::column().child(veloce_core::Text::new("MODAL")),
        )),
    };
    let placed = veloce_layout::resolve_rects(&el, 40, 10);
    for p in placed {
        println!("idx={} parent={} rect={:?}", p.index, p.parent, p.rect);
    }
    let grid = veloce_render::render_to_string(&el, 40, 10);
    println!("{grid}");
}
