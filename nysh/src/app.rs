mod commands;
pub mod config;
mod ui;
mod utils;

use config::AppConfig;

use ui::ui;

use std::io;

use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
};

pub enum CurrentScreen {
    Main,
    Exiting,
    NewSshIdentity,
}

pub struct App {
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    config: AppConfig,
) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|frame| ui(frame, app, config.clone()))?;

        if let Event::Key(key) = event::read()? {
            // Skip events that are not KeyEventKind::Press
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::NewSshIdentity;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    _ => {
                        app.current_screen = CurrentScreen::Main;
                    }
                },
                CurrentScreen::NewSshIdentity => match key.code {
                    _ => {
                        app.current_screen = CurrentScreen::Main;
                    }
                },
            }
        }
    }
}
