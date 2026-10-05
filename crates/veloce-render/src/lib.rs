use ratatui::buffer::Buffer;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType};

use veloce_core::{BorderStyle, Element};
use veloce_layout::resolve_rects;

pub fn render(element: &Element, buf: &mut Buffer) {
    let rects = resolve_rects(element, buf.area.width, buf.area.height);
    let mut it = rects.iter();
    render_node(element, buf, &mut it);
}

fn render_node(
    element: &Element,
    buf: &mut Buffer,
    rects: &mut std::slice::Iter<'_, veloce_layout::Placed>,
) {
    let abs = match rects.next() {
        Some(p) => p.rect,
        None => return,
    };
    match element {
        Element::Flex(f) => {
            if let Some(border) = f.style.border {
                let block = Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_type(match border {
                        BorderStyle::Rounded => BorderType::Rounded,
                        BorderStyle::Square => BorderType::Plain,
                    });
                ratatui::widgets::Widget::render(block, abs, buf);
            }
            for child in &f.children {
                render_node(child, buf, rects);
            }
        }
        Element::ScrollView(s) => {
            if let Some(border) = s.style.border {
                let block = Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_type(match border {
                        BorderStyle::Rounded => BorderType::Rounded,
                        BorderStyle::Square => BorderType::Plain,
                    });
                ratatui::widgets::Widget::render(block, abs, buf);
            }
            for child in &s.children {
                render_node(child, buf, rects);
            }
        }
        Element::Text(t) => {
            let mut style = Style::default();
            if t.bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            if let Some(c) = t.color {
                style = style.fg(c);
            }
            let max = abs.width as usize;
            buf.set_stringn(abs.x, abs.y, &t.content, max, style);
        }
        Element::Spacer(_) => {}
    }
}

/// Debug helper: renders an element tree into a TestBackend and returns the
/// grid as a string of rows joined by '\n'.
pub fn render_to_string(element: &Element, width: u16, height: u16) -> String {
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::TestBackend::new(width, height)).unwrap();
    terminal.draw(|f| render(element, f.buffer_mut())).unwrap();
    let buf = terminal.backend().buffer();
    let mut out = String::new();
    for y in 0..height {
        for x in 0..width {
            let cell = buf.cell((x, y)).unwrap();
            out.push_str(cell.symbol());
        }
        out.push('\n');
    }
    out
}
