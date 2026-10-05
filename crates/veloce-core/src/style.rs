#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDir {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    Rounded,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeSpec {
    Grow(f32),
    Fixed(u16),
    Auto,
}

/// Style attached to any element. Mapped to taffy::Style in veloce-layout.
#[derive(Debug, Clone)]
pub struct Style {
    pub direction: FlexDir,
    pub gap: u16,
    pub padding: u16,
    pub border: Option<BorderStyle>,
    pub grow: f32,
    pub shrink: f32,
    pub width: SizeSpec,
    pub height: SizeSpec,
    pub align_items: Option<Align>,
    pub justify_content: Option<Justify>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Start,
    Center,
    End,
    SpaceBetween,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            direction: FlexDir::Column,
            gap: 0,
            padding: 0,
            border: None,
            grow: 0.0,
            shrink: 1.0,
            width: SizeSpec::Auto,
            height: SizeSpec::Auto,
            align_items: None,
            justify_content: None,
        }
    }
}
