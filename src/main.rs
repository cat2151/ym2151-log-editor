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

    // Initialize server on Windows
    #[cfg(windows)]
    {
        ym2151_log_play_server::client::init_client(false); // false = not verbose
        if let Err(e) = ym2151_log_play_server::client::ensure_server_ready("cat-play-mml") {
            eprintln!("⚠️  Warning: Failed to ensure server is ready: {}", e);
            eprintln!("   Preview playback may not be available.");
        }
    }

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
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        app.preview_current_event();
                    }
                    KeyCode::Char(c @ '0'..='9') => {
                        // Map '1'-'9' to 1-9ms, '0' to 10ms
                        let milliseconds = match c {
                            '0' => 10,
                            c => c.to_digit(10).unwrap(),
                        };
                        app.set_wait_time_ms(milliseconds);
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
