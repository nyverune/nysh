pub mod modals;
pub mod ssh;
pub mod types;
mod ui;
mod utils;

use crate::config::AppConfig;
use ssh::commands::{get_ssh_identities::get_ssh_identities, types::SshIdentity};
use types::{CurrentModal, CurrentScreen};

use std::io;

use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
};

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_modal: CurrentModal,
    pub identities: Vec<SshIdentity>,
    pub selected_index: usize,
    pub exit: bool,
}

impl App {
    pub fn new(config: &AppConfig) -> App {
        App {
            current_screen: CurrentScreen::Main,
            current_modal: CurrentModal::NoModal,
            identities: get_ssh_identities(config),
            selected_index: 0,
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
