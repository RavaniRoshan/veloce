use ratatui::layout::Rect;
use veloce_core::{Element, Flex, Spacer, Text};
use veloce_layout::{resolve_rects, LayoutCache, Placed};

fn rects(placed: &[Placed]) -> Vec<Rect> {
    placed.iter().map(|p| p.rect).collect()
}

fn sample(long_text: bool) -> Element {
    Flex::column()
        .gap(1)
        .child(
            Flex::row()
                .child(Text::new("Service: auth-service-v2"))
                .child(Spacer::grow()),
        )
        .child(Text::new(if long_text {
            "a much longer status line that wraps differently"
        } else {
            "ok"
        }))
        .into_element()
}

#[test]
fn cached_layout_matches_fresh_layout() {
    let el = sample(false);
    let mut cache = LayoutCache::new();
    let first = rects(&cache.resolve(&el, 80, 24));
    let second = rects(&cache.resolve(&el, 80, 24));
    let fresh = rects(&resolve_rects(&el, 80, 24));
    assert_eq!(first, fresh);
    assert_eq!(second, fresh);
}

#[test]
fn text_change_invalidates_the_cache() {
    let before = sample(false);
    let after = sample(true);
    let mut cache = LayoutCache::new();
    let _ = cache.resolve(&before, 80, 24);
    let got = rects(&cache.resolve(&after, 80, 24));
    assert_eq!(got, rects(&resolve_rects(&after, 80, 24)));
}

#[test]
fn resize_invalidates_the_cache() {
    let el = sample(true);
    let mut cache = LayoutCache::new();
    let _ = cache.resolve(&el, 80, 24);
    let got = rects(&cache.resolve(&el, 40, 12));
    assert_eq!(got, rects(&resolve_rects(&el, 40, 12)));
}

#[test]
fn scroll_offset_invalidates_the_cache() {
    use veloce_core::ScrollView;

    let a = ScrollView::new()
        .child(Text::new("one"))
        .child(Text::new("two"))
        .into_element();
    let b = ScrollView::new()
        .offset(1)
        .child(Text::new("one"))
        .child(Text::new("two"))
        .into_element();
    let mut cache = LayoutCache::new();
    let _ = cache.resolve(&a, 40, 10);
    let got = rects(&cache.resolve(&b, 40, 10));
    assert_eq!(got, rects(&resolve_rects(&b, 40, 10)));
}
