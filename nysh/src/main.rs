mod app;
mod ui;

use crossterm::execute;
use crossterm::event::{
    DisableMouseCapture,
    EnableMouseCapture,
};
use crossterm::terminal::{
    enable_raw_mode,
    EnterAlternateScreen,
    disable_raw_mode,
    LeaveAlternateScreen
};

use std::{io, error};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = app::App::new();
    let res = app::run_app(&mut terminal, &mut app)?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

