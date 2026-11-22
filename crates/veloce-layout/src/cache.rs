use veloce_core::Element;

use crate::mirror::LayoutTree;
use crate::quantize::walk_rects;
use crate::Placed;

/// Reuses a computed `LayoutTree` while the element tree and terminal size are
/// unchanged. Veloce builds a fresh `Element` tree every frame, so the common
/// case is a hit: the taffy mirror and layout pass are skipped entirely.
///
/// A miss rebuilds and recomputes, then stores a clone of the new element tree
/// as the next cache key. Equality covers styles, text, scroll offset, and
/// children, so any visible change invalidates the cache.
pub struct LayoutCache {
    last: Option<Element>,
    tree: Option<LayoutTree>,
    size: (u16, u16),
}

impl Default for LayoutCache {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutCache {
    pub fn new() -> Self {
        Self {
            last: None,
            tree: None,
            size: (0, 0),
        }
    }

    /// Returns absolute quantized rects in pre-order.
    pub fn resolve(&mut self, root: &Element, cols: u16, rows: u16) -> Vec<Placed> {
        let size = (cols, rows);
        if self.size == size && self.last.as_ref() == Some(root) {
            if let Some(tree) = self.tree.as_ref() {
                return walk_rects(root, tree, cols, rows);
            }
        }

        let mut tree = LayoutTree::build(root);
        tree.compute(cols, rows);
        let out = walk_rects(root, &tree, cols, rows);
        self.tree = Some(tree);
        self.last = Some(root.clone());
        self.size = size;
        out
    }
}
