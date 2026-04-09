mod app;
mod cli;
mod event_editor;
mod file_io;
mod models;
mod navigation;
mod preview;
mod time_display;
mod ui;

#[cfg(test)]
mod tests;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn is_move_up_key(code: &KeyCode) -> bool {
    matches!(*code, KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K'))
}

fn is_move_down_key(code: &KeyCode) -> bool {
    matches!(
        *code,
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J')
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli::parse() {
        cli::StartupMode::Update => return cli::run_update(),
        cli::StartupMode::Check => return cli::run_check(),
        cli::StartupMode::Editor {
            use_clipboard,
            file_arg,
        } => run_editor(use_clipboard, file_arg)?,
    }

    Ok(())
}

fn run_editor(
    use_clipboard: bool,
    file_arg: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read clipboard content BEFORE terminal initialization so that on error
    // we can simply return without needing to restore the terminal.
    let clipboard_content = if use_clipboard {
        let text = arboard::Clipboard::new()
            .and_then(|mut cb| cb.get_text())
            .map_err(|e| format!("Failed to read clipboard: {}", e))?;
        Some(text)
    } else {
        None
    };

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

    // Create app and load data
    let mut app = App::new();
    if let Some(content) = clipboard_content {
        if let Err(e) = app.load_from_str(&content) {
            restore_terminal(&mut terminal)?;
            eprintln!("Error loading from clipboard: {}", e);
            return Err(e);
        }
    } else if let Some(path) = &file_arg {
        if let Err(e) = app.load_file(path) {
            restore_terminal(&mut terminal)?;
            eprintln!("Error loading file: {}", e);
            return Err(e);
        }
    }

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

/// Restore the terminal to its original state.
/// Should be called on both normal exit and error paths after `enable_raw_mode()` and
/// `EnterAlternateScreen` have been invoked. Returns an error if any restoration step fails.
fn restore_terminal<B: ratatui::backend::Backend + io::Write>(
    terminal: &mut Terminal<B>,
) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Only process key press events, not release events
                // This prevents double-triggering on Windows
                if key.kind == KeyEventKind::Press {
                    if app.help_visible() {
                        match key.code {
                            KeyCode::Char('?') | KeyCode::Esc => {
                                app.hide_help();
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') => {
                                app.should_quit = true;
                            }
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('?') => {
                            app.toggle_help();
                        }
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
                        KeyCode::Char('l') | KeyCode::Char('L') => {
                            app.toggle_loop();
                        }
                        KeyCode::Char(c @ '0'..='9') => {
                            // Map '0'-'9' to 0-9ms
                            let milliseconds = c.to_digit(10).unwrap();
                            app.set_wait_time_ms(milliseconds);
                        }
                        code if is_move_up_key(&code) => {
                            app.move_up();
                        }
                        code if is_move_down_key(&code) => {
                            app.move_down();
                        }
                        KeyCode::Delete => {
                            app.delete_selected_event();
                        }
                        KeyCode::Char('/') => {
                            app.insert_event_before_selected();
                        }
                        KeyCode::Enter => {
                            app.insert_event_before_selected();
                        }
                        _ => {}
                    }
                }
            }
        }

        app.tick();

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
