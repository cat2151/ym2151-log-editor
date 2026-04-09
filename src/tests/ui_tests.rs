use crate::app::App;
use crate::models::Ym2151Event;
use ratatui::{backend::TestBackend, style::Color, Terminal};

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

    let buffer = terminal.backend().buffer();
    let mut rendered = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            rendered.push_str(buffer[(x, y)].symbol());
        }
        rendered.push('\n');
    }

    assert!(rendered.contains("Clipboard JSON input: start with --clipboard"));
    assert!(rendered.contains("Help (? / ESC to close)"));
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
