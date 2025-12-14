use crate::app::{App, TimeDisplayMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Render the application UI
pub fn render(f: &mut Frame, app: &mut App) {
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

    let header_text = format!("{} | {}", title, time_mode_text);
    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(header, area);
}

fn render_content(f: &mut Frame, area: Rect, app: &mut App) {
    let visible_height = area.height.saturating_sub(2) as usize; // Account for borders

    // Update scroll offset to keep selected item visible
    app.update_scroll(visible_height);

    // Create list items for events, plus one empty line at the end
    let total_lines = app.log.events.len() + 1; // +1 for empty line after last event
    let items: Vec<ListItem> = (app.scroll_offset
        ..total_lines.min(app.scroll_offset + visible_height))
        .map(|i| {
            let content = if i < app.log.events.len() {
                app.format_event(i)
            } else {
                // Empty line for cursor positioning beyond last event
                String::new()
            };
            let style = if i == app.selected_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Events"));

    f.render_widget(list, area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let footer_text = if app.time_mode == TimeDisplayMode::Cumulative {
        vec![
            Span::raw("↑/↓: Navigate | "),
            Span::raw("1-0: Set Wait(ms) | "),
            Span::raw("/|ENTER: Insert | "),
            Span::raw("DEL: Delete | "),
            Span::raw("P: Preview | "),
            Span::raw("T: Toggle Time Mode | "),
            Span::raw("S: Save | "),
            Span::raw("Q/ESC: Quit"),
        ]
    } else {
        vec![
            Span::raw("↑/↓: Navigate | "),
            Span::raw("/|ENTER: Insert | "),
            Span::raw("DEL: Delete | "),
            Span::raw("P: Preview | "),
            Span::raw("T: Toggle Time Mode | "),
            Span::raw("S: Save | "),
            Span::raw("Q/ESC: Quit"),
        ]
    };

    let footer = Paragraph::new(Line::from(footer_text))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}
