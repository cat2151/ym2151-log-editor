mod app;
mod models;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get filename from command line args
    let args: Vec<String> = env::args().collect();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and load file if provided
    let mut app = App::new();
    if args.len() > 1 {
        if let Err(e) = app.load_file(&args[1]) {
            // Restore terminal before showing error
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            eprintln!("Error loading file: {}", e);
            return Err(e);
        }
    }

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if let Event::Key(key) = event::read()? {
            // Only process key press events, not release events
            // This prevents double-triggering on Windows
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        app.toggle_time_mode();
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        if let Err(e) = app.save_file() {
                            // In a real app, you'd want to show this error in the UI
                            eprintln!("Error saving file: {}", e);
                        }
                    }
                    KeyCode::Up => {
                        app.move_up();
                    }
                    KeyCode::Down => {
                        app.move_down();
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
