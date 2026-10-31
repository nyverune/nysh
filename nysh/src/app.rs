mod ssh;
pub mod types;
mod ui;
mod utils;

use crate::config::AppConfig;
use types::CurrentScreen;

use std::io;

use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
};

pub struct App {
    pub current_screen: CurrentScreen,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            exit: false,
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    config: &AppConfig,
) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    while !app.exit {
        terminal.draw(|frame| ui::draw(frame, app, config))?;

        if let Event::Key(key) = event::read()? {
            // Skip events that are not KeyEventKind::Press
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            if !ssh::handler::handler(app, key) {
                if let KeyCode::Char('c') = key.code {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        app.exit = true;
                    }
                }
            }
        }
    }

    Ok(true)
}
