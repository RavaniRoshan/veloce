use taffy::prelude::*;
use veloce_core::{Element, SizeSpec, Style};

/// Parallel to the Element tree: `nodes[i]` is the taffy node for the i-th
/// element in pre-order (DFS) traversal.
pub struct LayoutTree {
    pub tree: TaffyTree<String>,
    pub nodes: Vec<NodeId>,
}

impl LayoutTree {
    pub fn build(root: &Element) -> Self {
        let mut tree = TaffyTree::new();
        let mut nodes = Vec::new();
        build_node(&mut tree, &mut nodes, root);
        Self { tree, nodes }
    }

    pub fn root(&self) -> NodeId {
        self.nodes[0]
    }

    pub fn compute(&mut self, cols: u16, rows: u16) {
        let root = self.root();
        self.tree
            .compute_layout_with_measure(
                root,
                Size {
                    width: AvailableSpace::Definite(cols as f32),
                    height: AvailableSpace::Definite(rows as f32),
                },
                |_known, available_space, _node_id, ctx, _style| match ctx {
                    Some(text) if !text.is_empty() => {
                        let avail_w = match available_space.width {
                            AvailableSpace::Definite(w) => w,
                            _ => text.chars().count() as f32,
                        };
                        let chars = text.chars().count() as f32;
                        let width = avail_w.max(1.0).min(chars);
                        let height = (chars / width).ceil().max(1.0);
                        Size { width, height }
                    }
                    _ => Size::ZERO,
                },
            )
            .expect("taffy layout failed");
    }

    pub fn layout(&self, index: usize) -> &taffy::Layout {
        self.tree.layout(self.nodes[index]).expect("missing layout")
    }
}

fn build_node(tree: &mut TaffyTree<String>, nodes: &mut Vec<NodeId>, element: &Element) -> NodeId {
    let my_index = nodes.len();
    nodes.push(NodeId::from(0u64)); // placeholder keeps DFS indices aligned
    let id = match element {
        Element::Flex(f) => {
            let children: Vec<NodeId> = f
                .children
                .iter()
                .map(|c| build_node(tree, nodes, c))
                .collect();
            tree.new_with_children(to_taffy(&f.style, Some(element)), &children)
                .expect("taffy node")
        }
        Element::ScrollView(s) => {
            let children: Vec<NodeId> = s
                .children
                .iter()
                .map(|c| build_node(tree, nodes, c))
                .collect();
            tree.new_with_children(to_taffy(&s.style, Some(element)), &children)
                .expect("taffy node")
        }
        Element::Overlay {
            below: o_below,
            overlay: o_overlay,
        } => {
            let below = build_node(tree, nodes, o_below);
            let overlay = build_node(tree, nodes, o_overlay);
            tree.set_style(
                overlay,
                taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        top: taffy::LengthPercentageAuto::percent(0.2),
                        left: taffy::LengthPercentageAuto::percent(0.2),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    size: taffy::Size {
                        width: taffy::Dimension::percent(0.6),
                        height: taffy::Dimension::percent(0.6),
                    },
                    display: taffy::Display::Flex,
                    ..Default::default()
                },
            )
            .expect("set style");
            tree.new_with_children(
                taffy::Style {
                    display: taffy::Display::Flex,
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),
                        height: taffy::Dimension::percent(1.0),
                    },
                    ..Default::default()
                },
                &[below, overlay],
            )
            .expect("taffy node")
        }
        Element::Modal(m) => {
            let below = tree
                .new_leaf(taffy::Style {
                    display: taffy::Display::Flex,
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),
                        height: taffy::Dimension::percent(1.0),
                    },
                    ..Default::default()
                })
                .expect("taffy node");
            let overlay = build_node(tree, nodes, &m.content);
            tree.set_style(
                overlay,
                taffy::Style {
                    position: taffy::Position::Absolute,
                    inset: taffy::Rect {
                        top: taffy::LengthPercentageAuto::percent(0.2),
                        left: taffy::LengthPercentageAuto::percent(0.2),
                        right: taffy::LengthPercentageAuto::auto(),
                        bottom: taffy::LengthPercentageAuto::auto(),
                    },
                    size: taffy::Size {
                        width: taffy::Dimension::percent(0.6),
                        height: taffy::Dimension::percent(0.6),
                    },
                    display: taffy::Display::Flex,
                    ..Default::default()
                },
            )
            .expect("set style");
            tree.new_with_children(
                taffy::Style {
                    display: taffy::Display::Flex,
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),
                        height: taffy::Dimension::percent(1.0),
                    },
                    ..Default::default()
                },
                &[below, overlay],
            )
            .expect("taffy node")
        }
        Element::Text(t) => tree
            .new_leaf_with_context(
                taffy::Style {
                    display: taffy::Display::Flex,
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                t.content.clone(),
            )
            .expect("taffy node"),
        Element::TextInput(ti) => tree
            .new_leaf(taffy::Style {
                display: taffy::Display::Flex,
                size: taffy::Size {
                    width: taffy::Dimension::length(ti.content.chars().count().max(10) as f32),
                    height: taffy::Dimension::length(1.0),
                },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .expect("taffy node"),
        Element::Spacer(_) => tree
            .new_leaf(to_taffy(&Style::default(), Some(element)))
            .expect("taffy node"),
    };
    nodes[my_index] = id;
    id
}

fn to_taffy(style: &Style, element: Option<&Element>) -> taffy::Style {
    let mut s = taffy::Style {
        display: Display::Flex,
        ..Default::default()
    };
    s.flex_direction = match style.direction {
        veloce_core::FlexDir::Row => taffy::FlexDirection::Row,
        veloce_core::FlexDir::Column => taffy::FlexDirection::Column,
    };
    s.gap = Size::from_length(style.gap as f32);
    let pad = LengthPercentage::length(style.padding as f32);
    s.padding = Rect {
        left: pad,
        right: pad,
        top: pad,
        bottom: pad,
    };
    let b = match style.border {
        Some(_) => LengthPercentage::length(1.0),
        None => LengthPercentage::length(0.0),
    };
    s.border = Rect {
        left: b,
        right: b,
        top: b,
        bottom: b,
    };
    s.flex_grow = style.grow;
    s.flex_shrink = style.shrink;
    s.size = Size {
        width: match style.width {
            SizeSpec::Fixed(n) => Dimension::length(n as f32),
            SizeSpec::Grow(_) => Dimension::auto(),
            SizeSpec::Auto => match element {
                Some(Element::Flex(_)) | Some(Element::ScrollView(_)) => Dimension::percent(1.0),
                _ => Dimension::auto(),
            },
        },
        height: match style.height {
            SizeSpec::Fixed(n) => Dimension::length(n as f32),
            SizeSpec::Grow(_) => Dimension::auto(),
            SizeSpec::Auto => match element {
                Some(Element::Flex(_)) | Some(Element::ScrollView(_)) => Dimension::percent(1.0),
                _ => Dimension::auto(),
            },
        },
    };
    if let Some(a) = style.align_items {
        s.align_items = Some(match a {
            veloce_core::Align::Start => AlignItems::Start,
            veloce_core::Align::Center => AlignItems::Center,
            veloce_core::Align::End => AlignItems::End,
            veloce_core::Align::Stretch => AlignItems::Stretch,
        })
    }
    if let Some(j) = style.justify_content {
        s.justify_content = Some(match j {
            veloce_core::Justify::Start => JustifyContent::Start,
            veloce_core::Justify::Center => JustifyContent::Center,
            veloce_core::Justify::End => JustifyContent::End,
            veloce_core::Justify::SpaceBetween => JustifyContent::SpaceBetween,
        })
    }
    match element {
        Some(Element::Text(t)) => {
            s.size = Size {
                width: Dimension::length(t.content.len() as f32),
                height: Dimension::length(1.0),
            };
            s.flex_shrink = 0.0;
        }
        Some(Element::Spacer(sp)) => {
            s.flex_grow = sp.grow;
            s.flex_shrink = 1.0;
        }
        _ => {}
    }
    s
}
