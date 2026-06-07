use crate::app::App;
use crate::models::Ym2151Event;
use crate::time_display::TimeDisplayMode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

const MONOKAI_BG: Color = Color::Rgb(39, 40, 34);
const MONOKAI_FG: Color = Color::Rgb(248, 248, 242);
const MONOKAI_COMMENT: Color = Color::Rgb(117, 113, 94);
const MONOKAI_CYAN: Color = Color::Rgb(102, 217, 239);
const MONOKAI_ORANGE: Color = Color::Rgb(253, 151, 31);
const MONOKAI_PINK: Color = Color::Rgb(249, 38, 114);
const MONOKAI_PURPLE: Color = Color::Rgb(174, 129, 255);
const MONOKAI_YELLOW: Color = Color::Rgb(230, 219, 116);
const MONOKAI_SELECTION: Color = Color::Rgb(73, 72, 62);
const HELP_HORIZONTAL_PADDING: u16 = 4;
const HELP_VERTICAL_PADDING: u16 = 2;
const POPUP_MARGIN: u16 = 2;
const MIN_POPUP_SIZE: u16 = 3;

/// Render the application UI
pub fn render(f: &mut Frame, app: &mut App) {
    f.render_widget(
        Block::default().style(Style::default().bg(MONOKAI_BG)),
        f.area(),
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_content(f, chunks[1], app);
    render_footer(f, chunks[2], app);

    if app.help_visible() {
        render_help_overlay(f);
    }
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let title = if let Some(path) = &app.file_path {
        format!("YM2151 Log Editor - {}", path)
    } else {
        String::from("YM2151 Log Editor - No file loaded")
    };

    let time_mode_text = match app.time_mode {
        TimeDisplayMode::Cumulative => "Time: Cumulative",
        TimeDisplayMode::Timestamp => "Time: Timestamp",
    };

    let loop_text = if app.loop_enabled {
        "Loop: On"
    } else {
        "Loop: Off"
    };

    let header_text = format!("{} | {} | {}", title, time_mode_text, loop_text);
    let header = Paragraph::new(header_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(MONOKAI_PURPLE))
                .style(Style::default().bg(MONOKAI_BG)),
        )
        .style(Style::default().fg(MONOKAI_CYAN).bg(MONOKAI_BG));

    f.render_widget(header, area);
}

fn render_content(f: &mut Frame, area: Rect, app: &mut App) {
    let visible_height = area.height.saturating_sub(2) as usize; // Account for borders

    // Update scroll offset to keep selected item visible
    app.update_scroll(visible_height);

    // Create list items for events, plus one empty line at the end
    let total_lines = app.log.events.len() + 1; // +1 for empty line after last event
    let items: Vec<ListItem> = (app.scroll_offset()
        ..total_lines.min(app.scroll_offset() + visible_height))
        .map(|i| {
            let selected = i == app.selected_index();
            let content = if i < app.log.events.len() {
                event_line(app, i, selected)
            } else {
                // Empty line for cursor positioning beyond last event
                Line::from(String::new())
            };
            let style = event_row_style(selected);
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Events")
            .border_style(Style::default().fg(MONOKAI_PURPLE))
            .style(Style::default().bg(MONOKAI_BG)),
    );

    f.render_widget(list, area);
}

fn event_row_style(selected: bool) -> Style {
    if selected {
        Style::default()
            .fg(MONOKAI_YELLOW)
            .bg(MONOKAI_SELECTION)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(MONOKAI_FG).bg(MONOKAI_BG)
    }
}

fn event_line(app: &App, index: usize, selected: bool) -> Line<'static> {
    let event = &app.log.events[index];
    let formatted = app.format_event(index);
    let time = crate::time_display::get_time_string(&app.log, index, app.time_mode);

    let mut cursor = 0;
    let mut spans = Vec::with_capacity(9);
    push_formatted_span(
        &mut spans,
        &formatted,
        &mut cursor,
        time.len(),
        syntax_style(MONOKAI_COMMENT, selected),
    );
    push_formatted_span(
        &mut spans,
        &formatted,
        &mut cursor,
        2,
        syntax_style(MONOKAI_COMMENT, selected),
    );
    push_formatted_span(
        &mut spans,
        &formatted,
        &mut cursor,
        event.addr.len(),
        syntax_style(MONOKAI_PURPLE, selected),
    );
    push_formatted_span(
        &mut spans,
        &formatted,
        &mut cursor,
        2,
        syntax_style(MONOKAI_COMMENT, selected),
    );
    push_formatted_span(
        &mut spans,
        &formatted,
        &mut cursor,
        event.data.len(),
        syntax_style(MONOKAI_CYAN, selected),
    );
    push_formatted_span(
        &mut spans,
        &formatted,
        &mut cursor,
        2,
        syntax_style(MONOKAI_COMMENT, selected),
    );
    if let Some(description) = formatted.get(cursor..) {
        spans.extend(description_spans(event, description, selected));
    }

    Line::from(spans)
}

fn push_formatted_span(
    spans: &mut Vec<Span<'static>>,
    formatted: &str,
    cursor: &mut usize,
    len: usize,
    style: Style,
) {
    let end = *cursor + len;
    if let Some(text) = formatted.get(*cursor..end) {
        spans.push(Span::styled(text.to_string(), style));
        *cursor = end;
    }
}

fn description_spans(event: &Ym2151Event, description: &str, selected: bool) -> Vec<Span<'static>> {
    if is_key_on_event(event, description) {
        highlighted_token_spans(description, "keyon", selected)
    } else {
        vec![Span::styled(
            description.to_string(),
            syntax_style(MONOKAI_FG, selected),
        )]
    }
}

fn is_key_on_event(event: &Ym2151Event, description: &str) -> bool {
    event.is_key_on() && !event.is_key_off() && description.contains("keyon")
}

fn highlighted_token_spans(text: &str, token: &str, selected: bool) -> Vec<Span<'static>> {
    let Some(start) = text.find(token) else {
        return vec![Span::styled(
            text.to_string(),
            syntax_style(MONOKAI_FG, selected),
        )];
    };

    let end = start + token.len();
    let mut spans = Vec::with_capacity(3);
    if start > 0 {
        spans.push(Span::styled(
            text[..start].to_string(),
            syntax_style(MONOKAI_FG, selected),
        ));
    }
    spans.push(Span::styled(token.to_string(), key_on_style(selected)));
    if end < text.len() {
        spans.push(Span::styled(
            text[end..].to_string(),
            syntax_style(MONOKAI_FG, selected),
        ));
    }
    spans
}

fn syntax_style(fg: Color, selected: bool) -> Style {
    let style = Style::default().fg(fg).bg(if selected {
        MONOKAI_SELECTION
    } else {
        MONOKAI_BG
    });

    if selected {
        style.add_modifier(Modifier::BOLD)
    } else {
        style
    }
}

fn key_on_style(selected: bool) -> Style {
    syntax_style(MONOKAI_PINK, selected)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::UNDERLINED)
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let footer_text = if app.time_mode == TimeDisplayMode::Cumulative {
        vec![
            Span::raw("Esc/q: Quit, ?: Help | "),
            Span::raw("↑/↓ or k/j: Navigate | "),
            Span::raw("0-9: Set Wait(ms) | "),
            Span::raw("/|ENTER: Insert | "),
            Span::raw("DEL: Delete | "),
            Span::raw("P: Preview | "),
            Span::raw("L: Loop On/Off | "),
            Span::raw("T: Toggle Time Mode | "),
            Span::raw("S: Save | "),
        ]
    } else {
        vec![
            Span::raw("Esc/q: Quit, ?: Help | "),
            Span::raw("↑/↓ or k/j: Navigate | "),
            Span::raw("/|ENTER: Insert | "),
            Span::raw("DEL: Delete | "),
            Span::raw("P: Preview | "),
            Span::raw("L: Loop On/Off | "),
            Span::raw("T: Toggle Time Mode | "),
            Span::raw("S: Save | "),
        ]
    };

    let footer = Paragraph::new(Line::from(footer_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(MONOKAI_PURPLE))
                .style(Style::default().bg(MONOKAI_BG)),
        )
        .style(Style::default().fg(MONOKAI_COMMENT).bg(MONOKAI_BG));

    f.render_widget(footer, area);
}

fn render_help_overlay(f: &mut Frame) {
    let help_lines = help_lines();
    let popup = help_popup_rect(f.area());
    let text = help_lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            let style = if index == 0 {
                Style::default()
                    .fg(MONOKAI_ORANGE)
                    .bg(MONOKAI_BG)
                    .add_modifier(Modifier::BOLD)
            } else if line.contains("clipboard") {
                Style::default().fg(MONOKAI_CYAN).bg(MONOKAI_BG)
            } else {
                Style::default().fg(MONOKAI_FG).bg(MONOKAI_BG)
            };
            Line::from(Span::styled(line, style))
        })
        .collect::<Vec<_>>();
    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(MONOKAI_ORANGE))
        .style(Style::default().bg(MONOKAI_BG));

    f.render_widget(Clear, popup);
    f.render_widget(Paragraph::new(text).block(block), popup);
}

fn help_lines() -> [&'static str; 17] {
    [
        "Help (? / Esc to close)",
        "",
        "↑/↓ or k/j: Navigate",
        "PgUp / PgDn: Jump 10 lines",
        "Ctrl+U / Ctrl+D: Jump 10 lines",
        "[count]k / [count]j: Move by count (vim style)",
        "0-9: Set Wait(ms) in cumulative mode",
        "/ or ENTER: Insert event before cursor",
        "DEL: Delete selected event",
        "P: Preview current JSON",
        "L: Toggle loop playback",
        "I: Import JSON from clipboard",
        "T: Toggle time display mode",
        "S: Save file",
        "Esc / q: Quit editor",
        "",
        "Clipboard JSON input: start with --clipboard",
    ]
}

fn help_popup_rect(area: Rect) -> Rect {
    let help_lines = help_lines();
    let content_width = widest_display_width(&help_lines);
    centered_rect(
        content_width + HELP_HORIZONTAL_PADDING,
        help_lines.len() as u16 + HELP_VERTICAL_PADDING,
        area,
    )
}

fn widest_display_width(lines: &[&str]) -> u16 {
    lines
        .iter()
        .map(|line| UnicodeWidthStr::width(*line) as u16)
        .max()
        .unwrap_or(0)
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let available_width = area.width.saturating_sub(POPUP_MARGIN);
    let available_height = area.height.saturating_sub(POPUP_MARGIN);
    let min_popup_width = MIN_POPUP_SIZE.min(available_width);
    let min_popup_height = MIN_POPUP_SIZE.min(available_height);
    let popup_width = width.min(available_width).max(min_popup_width);
    let popup_height = height.min(available_height).max(min_popup_height);

    Rect {
        x: area.x + area.width.saturating_sub(popup_width) / 2,
        y: area.y + area.height.saturating_sub(popup_height) / 2,
        width: popup_width,
        height: popup_height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widest_display_width_uses_terminal_width_not_utf8_bytes() {
        let lines = ["abc", "↑/↓"];

        assert_eq!(
            widest_display_width(&lines),
            UnicodeWidthStr::width("↑/↓") as u16
        );
    }

    #[test]
    fn centered_rect_never_exceeds_small_area() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 2,
            height: 2,
        };
        let popup = centered_rect(20, 20, area);

        assert!(popup.width <= area.width);
        assert!(popup.height <= area.height);
    }
}
