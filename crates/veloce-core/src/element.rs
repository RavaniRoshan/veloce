use crate::style::{BorderStyle, SizeSpec, Style};

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Flex(Flex),
    Text(Text),
    Spacer(Spacer),
    ScrollView(ScrollView),
    Overlay {
        below: Box<Element>,
        overlay: Box<Element>,
    },
    TextInput(TextInput),
    Modal(Modal),
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Overlay {
    pub below: Box<Element>,
    pub overlay: Box<Element>,
}

impl Overlay {
    pub fn new(below: impl Into<Element>, overlay: impl Into<Element>) -> Self {
        Self {
            below: Box::new(below.into()),
            overlay: Box::new(overlay.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScrollView {
    pub style: Style,
    pub children: Vec<Element>,
    pub offset: u16,
}

impl ScrollView {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
            children: Vec::new(),
            offset: 0,
        }
    }

    pub fn offset(mut self, offset: u16) -> Self {
        self.offset = offset;
        self
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

#[derive(Debug, Clone, PartialEq)]
pub struct TextInput {
    pub content: String,
    pub cursor: usize, // char index
    pub placeholder: Option<String>,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor: 0,
            placeholder: None,
        }
    }

    pub fn with_text(content: impl Into<String>) -> Self {
        let c = content.into();
        let len = c.chars().count();
        Self {
            content: c,
            cursor: len,
            placeholder: None,
        }
    }

    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = Some(p.into());
        self
    }

    /// Returns true when the key mutates the field's state.
    pub fn handle_key(&mut self, key: &crossterm::event::KeyEvent) -> bool {
        use crossterm::event::KeyCode::*;
        match &key.code {
            Char(c) => {
                let byte_index: usize = self
                    .content
                    .char_indices()
                    .nth(self.cursor)
                    .map(|(i, _)| i)
                    .unwrap_or(self.content.len());
                self.content.insert(byte_index, *c);
                self.cursor += 1;
                true
            }
            Backspace => {
                if self.cursor > 0 {
                    let byte_index: usize = self
                        .content
                        .char_indices()
                        .nth(self.cursor - 1)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    let byte_len = self
                        .content
                        .chars()
                        .nth(self.cursor - 1)
                        .map(|c| c.len_utf8())
                        .unwrap_or(0);
                    self.content
                        .replace_range(byte_index..byte_index + byte_len, "");
                    self.cursor -= 1;
                    true
                } else {
                    false
                }
            }
            Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                true
            }
            Right => {
                let len = self.content.chars().count();
                if self.cursor < len {
                    self.cursor += 1;
                }
                true
            }
            Home => {
                self.cursor = 0;
                true
            }
            End => {
                self.cursor = self.content.chars().count();
                true
            }
            _ => false,
        }
    }

    pub fn into_element(self) -> Element {
        Element::TextInput(self)
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TextInput> for Element {
    fn from(t: TextInput) -> Self {
        Element::TextInput(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modal {
    pub content: Box<Element>,
}

impl Modal {
    pub fn new(content: impl Into<Element>) -> Self {
        Self {
            content: Box::new(content.into()),
        }
    }

    pub fn into_element(self) -> Element {
        Element::Modal(self)
    }
}

impl From<Modal> for Element {
    fn from(m: Modal) -> Self {
        Element::Modal(m)
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

impl From<Overlay> for Element {
    fn from(o: Overlay) -> Self {
        Element::Overlay {
            below: o.below,
            overlay: o.overlay,
        }
    }
}

impl From<Spacer> for Element {
    fn from(s: Spacer) -> Self {
        Element::Spacer(s)
    }
}
