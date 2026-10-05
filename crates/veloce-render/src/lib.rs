use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType};

use veloce_core::{BorderStyle, Element};
use veloce_layout::{resolve_rects, Placed};

pub fn render(element: &Element, buf: &mut Buffer) {
    let rects = resolve_rects(element, buf.area.width, buf.area.height);
    let mut i = 0;
    render_node(element, buf, &rects, &mut i);
}

fn render_node(element: &Element, buf: &mut Buffer, rects: &[Placed], i: &mut usize) -> Rect {
    let abs = rects[*i].rect;
    *i += 1;
    match element {
        Element::Overlay { below, overlay } => {
            render_node(below, buf, rects, i);
            let modal_abs = rects[*i].rect;
            let y0 = abs.y;
            let x0 = abs.x;
            let y1 = abs.y + abs.height;
            let x1 = abs.x + abs.width;
            for y in y0..y1 {
                for x in x0..x1 {
                    let in_modal = x >= modal_abs.x
                        && x < modal_abs.x + modal_abs.width
                        && y >= modal_abs.y
                        && y < modal_abs.y + modal_abs.height;
                    if !in_modal {
                        if let Some(cell) = buf.cell_mut((x, y)) {
                            // Dimming: keep the underlying symbol, darken the fg.
                            let sym = cell.symbol().to_string();
                            cell.set_symbol(if sym.trim().is_empty() { "░" } else { &sym });
                            cell.set_style(Style::default().fg(Color::DarkGray));
                        }
                    }
                }
            }
            render_node(overlay, buf, rects, i);
            abs
        }
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
                render_node(child, buf, rects, i);
            }
            abs
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
                render_node(child, buf, rects, i);
            }
            abs
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
            abs
        }
        Element::Spacer(_) => abs,
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
