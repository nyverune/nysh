use crate::ui::ui;

use std::io;

use ratatui::{Terminal, backend::Backend};

pub enum CurrentScreen {
    Main,
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

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|frame| ui(frame, app))?;
    }
}
