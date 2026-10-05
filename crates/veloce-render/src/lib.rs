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
            dim_around(buf, abs, modal_abs);
            render_node(overlay, buf, rects, i);
            abs
        }
        Element::Modal(m) => {
            let modal_abs = rects[*i].rect;
            dim_around(buf, abs, modal_abs);
            render_node(&m.content, buf, rects, i);
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
            let viewport_top = abs.y as i32;
            let viewport_bottom = (abs.y + abs.height) as i32;
            let mut total_height: u16 = 0;
            let mut row_y: u16 = 0;
            for child in s.children.iter() {
                // track height to derive total for scrollbar
                let child_height = match child {
                    Element::Text(_) => 1u16,
                    Element::TextInput(_) => 1,
                    Element::Spacer(_) => 1,
                    _ => 1,
                };
                total_height = total_height.saturating_add(child_height);
                let translated_top = viewport_top + row_y as i32 - s.offset as i32;
                let translated_bottom = translated_top + child_height as i32;
                row_y = row_y.saturating_add(child_height);
                let _placed = &rects[*i];
                *i += 1;
                // child index placed at child site must correspond
                if translated_bottom <= viewport_top || translated_top >= viewport_bottom {
                    continue;
                }
                match child {
                    Element::Text(t) => {
                        let y = translated_top.max(viewport_top) as u16;
                        let mut style = Style::default();
                        if t.bold {
                            style = style.add_modifier(Modifier::BOLD);
                        }
                        if let Some(c) = t.color {
                            style = style.fg(c);
                        }
                        buf.set_stringn(abs.x, y, &t.content, abs.width as usize, style);
                    }
                    Element::TextInput(ti) => {
                        let y = translated_top as u16;
                        buf.set_stringn(
                            abs.x,
                            y,
                            &ti.content,
                            abs.width as usize,
                            Style::default(),
                        );
                    }
                    _ => {
                        // unsupported element types in scroll view: draw placeholder
                        let y = translated_top.max(viewport_top) as u16;
                        buf.set_stringn(abs.x, y, "…", abs.width as usize, Style::default());
                    }
                }
            }
            // Scrollbar
            if total_height > abs.height && abs.width > 0 && abs.height > 0 {
                let usable_overflow = (total_height - abs.height).max(1);
                let pos = s
                    .offset
                    .min(usable_overflow)
                    .saturating_mul(abs.height.saturating_sub(1))
                    / usable_overflow;
                for y in 0..abs.height {
                    if let Some(cell) = buf.cell_mut((abs.x + abs.width - 1, abs.y + y)) {
                        cell.set_symbol(if y == pos { "█" } else { "│" });
                        cell.set_style(Style::default().fg(Color::DarkGray));
                    }
                }
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
            for (row, line) in wrap_text(&t.content, max).iter().enumerate() {
                let y = abs.y + row as u16;
                if y >= abs.y + abs.height {
                    break;
                }
                buf.set_stringn(abs.x, y, line, max, style);
            }
            abs
        }
        Element::TextInput(ti) => {
            let content = if ti.content.is_empty() {
                ti.placeholder.clone().unwrap_or_default()
            } else {
                ti.content.clone()
            };
            let mut style = Style::default();
            if ti.content.is_empty() {
                style = style.fg(Color::DarkGray);
            }
            buf.set_stringn(abs.x, abs.y, &content, abs.width as usize, style);
            if abs.width > 0 {
                let col = ti.cursor.min(abs.width.saturating_sub(1) as usize) as u16;
                if let Some(cell) = buf.cell_mut((abs.x + col, abs.y)) {
                    cell.set_style(Style::default().fg(Color::Black).bg(Color::White));
                }
            }
            abs
        }
        Element::Spacer(_) => abs,
    }
}

fn dim_around(buf: &mut Buffer, abs: Rect, modal_abs: Rect) {
    for y in abs.y..abs.y + abs.height {
        for x in abs.x..abs.x + abs.width {
            let in_modal = x >= modal_abs.x
                && x < modal_abs.x + modal_abs.width
                && y >= modal_abs.y
                && y < modal_abs.y + modal_abs.height;
            if !in_modal {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_symbol("░");
                    cell.set_style(Style::default().fg(Color::DarkGray));
                }
            }
        }
    }
}

fn wrap_text(s: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![s.to_string()];
    }
    let mut out = Vec::new();
    for raw_line in s.lines() {
        let mut start = 0;
        let chars: Vec<char> = raw_line.chars().collect();
        while start < chars.len() {
            let end = (start + width).min(chars.len());
            out.push(chars[start..end].iter().collect());
            start = end;
        }
        if chars.is_empty() {
            out.push(String::new());
        }
    }
    out
}

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
