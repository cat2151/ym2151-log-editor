use crate::app::App;
use crate::models::Ym2151Event;
use ratatui::{
    backend::TestBackend,
    style::{Color, Modifier},
    Terminal,
};

fn render_to_string(terminal: &Terminal<TestBackend>) -> String {
    let buffer = terminal.backend().buffer();
    let mut rendered = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            rendered.push_str(buffer[(x, y)].symbol());
        }
        rendered.push('\n');
    }
    rendered
}

fn find_text_position(terminal: &Terminal<TestBackend>, needle: &str) -> Option<(u16, u16)> {
    let buffer = terminal.backend().buffer();
    for y in 0..buffer.area.height {
        let mut row = String::new();
        for x in 0..buffer.area.width {
            row.push_str(buffer[(x, y)].symbol());
        }
        if let Some(byte_index) = row.find(needle) {
            return Some((row[..byte_index].chars().count() as u16, y));
        }
    }
    None
}

#[test]
fn help_overlay_renders_clipboard_hint() {
    let backend = TestBackend::new(90, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.log.events = vec![Ym2151Event {
        time: 0.0,
        addr: "20".to_string(),
        data: "4F".to_string(),
    }];
    app.toggle_help();

    terminal.draw(|f| crate::ui::render(f, &mut app)).unwrap();

    let rendered = render_to_string(&terminal);

    assert!(rendered.contains("Clipboard JSON input: start with --clipboard"));
    assert!(rendered.contains("Help (? / Esc to close)"));
    assert!(rendered.contains("Esc / q: Quit editor"));
    assert!(rendered.contains("↑/↓ or k/j: Navigate"));
    assert!(rendered.contains("PgUp / PgDn: Jump 10 lines"));
    assert!(rendered.contains("Ctrl+U / Ctrl+D: Jump 10 lines"));
    assert!(rendered.contains("[count]k / [count]j: Move by count (vim style)"));
    assert!(rendered.contains("I: Import JSON from clipboard"));
}

#[test]
fn monokai_background_fills_screen() {
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();

    terminal.draw(|f| crate::ui::render(f, &mut app)).unwrap();

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer[(0, 0)].bg, Color::Rgb(39, 40, 34));
}

#[test]
fn footer_uses_zero_to_nine_wait_hint() {
    let backend = TestBackend::new(120, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();

    terminal.draw(|f| crate::ui::render(f, &mut app)).unwrap();

    let rendered = render_to_string(&terminal);

    assert!(rendered.contains("Esc/q: Quit, ?: Help"));
    assert!(rendered.contains("0-9: Set Wait(ms)"));
    assert!(!rendered.contains("1-0: Set Wait(ms)"));
    assert!(rendered.contains("↑/↓ or k/j: Navigate"));
}

#[test]
fn footer_uses_vim_style_navigation_hint_in_timestamp_mode() {
    let backend = TestBackend::new(120, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.toggle_time_mode();

    terminal.draw(|f| crate::ui::render(f, &mut app)).unwrap();

    let rendered = render_to_string(&terminal);

    assert!(rendered.contains("Esc/q: Quit, ?: Help"));
    assert!(rendered.contains("↑/↓ or k/j: Navigate"));
    assert!(!rendered.contains("0-9: Set Wait(ms)"));
}

#[test]
fn event_list_renders_description_column_content() {
    let backend = TestBackend::new(120, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.log.events = vec![Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "78".to_string(),
    }];

    terminal.draw(|f| crate::ui::render(f, &mut app)).unwrap();

    let rendered = render_to_string(&terminal);

    assert!(rendered.contains("0.000000  08  78  ch0 keyon"));
}

#[test]
fn event_list_highlights_keyon_as_strongest_monokai_token() {
    let backend = TestBackend::new(120, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.log.events = vec![Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "78".to_string(),
    }];

    terminal.draw(|f| crate::ui::render(f, &mut app)).unwrap();

    let buffer = terminal.backend().buffer();
    let (x, y) = find_text_position(&terminal, "keyon").unwrap();
    let keyon_cell = &buffer[(x, y)];

    assert_eq!(keyon_cell.fg, Color::Rgb(249, 38, 114));
    assert_eq!(keyon_cell.bg, Color::Rgb(73, 72, 62));
    assert!(keyon_cell.modifier.contains(Modifier::BOLD));
    assert!(keyon_cell.modifier.contains(Modifier::UNDERLINED));
}

#[test]
fn event_list_does_not_apply_keyon_highlight_to_keyoff() {
    let backend = TestBackend::new(120, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.log.events = vec![Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "00".to_string(),
    }];

    terminal.draw(|f| crate::ui::render(f, &mut app)).unwrap();

    let buffer = terminal.backend().buffer();
    let (x, y) = find_text_position(&terminal, "keyoff").unwrap();
    let keyoff_cell = &buffer[(x, y)];

    assert_ne!(keyoff_cell.fg, Color::Rgb(249, 38, 114));
    assert!(!keyoff_cell.modifier.contains(Modifier::UNDERLINED));
}

#[test]
fn event_list_renders_description_column_content_for_prefixed_hex_json_values() {
    let backend = TestBackend::new(120, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.log.events = vec![Ym2151Event {
        time: 0.0,
        addr: "0xA8".to_string(),
        data: "0x05".to_string(),
    }];

    terminal.draw(|f| crate::ui::render(f, &mut app)).unwrap();

    let rendered = render_to_string(&terminal);

    assert!(rendered.contains("0.000000  0xA8  0x05  ch0 c1 am enable / decay1 rate"));
}
