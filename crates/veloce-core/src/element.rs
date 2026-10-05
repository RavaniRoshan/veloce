use crate::style::{BorderStyle, SizeSpec, Style};

#[derive(Debug, Clone)]
pub enum Element {
    Flex(Flex),
    Text(Text),
    Spacer(Spacer),
    ScrollView(ScrollView),
}

#[derive(Debug, Clone)]
pub struct Flex {
    pub style: Style,
    pub children: Vec<Element>,
}

impl Flex {
    pub fn row() -> Self {
        let mut style = Style::default();
        style.direction = crate::style::FlexDir::Row;
        Self {
            style,
            children: Vec::new(),
        }
    }

    pub fn column() -> Self {
        Self {
            style: Style::default(),
            children: Vec::new(),
        }
    }

    pub fn gap(mut self, gap: u16) -> Self {
        self.style.gap = gap;
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.style.padding = padding;
        self
    }

    pub fn border(mut self, border: BorderStyle) -> Self {
        self.style.border = Some(border);
        self
    }

    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.style.grow = grow;
        self
    }

    pub fn width(mut self, w: SizeSpec) -> Self {
        self.style.width = w;
        self
    }

    pub fn height(mut self, h: SizeSpec) -> Self {
        self.style.height = h;
        self
    }

    pub fn child(mut self, child: impl Into<Element>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, iter: impl IntoIterator<Item = impl Into<Element>>) -> Self {
        self.children.extend(iter.into_iter().map(Into::into));
        self
    }

    pub fn into_element(self) -> Element {
        Element::Flex(self)
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
    pub bold: bool,
    pub color: Option<ratatui::style::Color>,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            bold: false,
            color: None,
        }
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn color(mut self, color: ratatui::style::Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn into_element(self) -> Element {
        Element::Text(self)
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Text::new(s)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Spacer {
    pub grow: f32,
}

impl Spacer {
    pub fn grow() -> Self {
        Self { grow: 1.0 }
    }

    pub fn into_element(self) -> Element {
        Element::Spacer(self)
    }
}

#[derive(Debug, Clone)]
pub struct ScrollView {
    pub style: Style,
    pub children: Vec<Element>,
}

impl ScrollView {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
            children: Vec::new(),
        }
    }

    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.style.grow = grow;
        self
    }

    pub fn child(mut self, child: impl Into<Element>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, iter: impl IntoIterator<Item = impl Into<Element>>) -> Self {
        self.children.extend(iter.into_iter().map(Into::into));
        self
    }

    pub fn into_element(self) -> Element {
        Element::ScrollView(self)
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Flex> for Element {
    fn from(f: Flex) -> Self {
        Element::Flex(f)
    }
}

impl From<Text> for Element {
    fn from(t: Text) -> Self {
        Element::Text(t)
    }
}

impl From<ScrollView> for Element {
    fn from(s: ScrollView) -> Self {
        Element::ScrollView(s)
    }
}

impl From<Spacer> for Element {
    fn from(s: Spacer) -> Self {
        Element::Spacer(s)
    }
}
