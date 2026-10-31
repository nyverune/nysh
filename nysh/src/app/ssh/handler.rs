use crate::app::{App, CurrentScreen};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn handler(app: &mut App, key: KeyEvent) -> bool {
    match app.current_screen {
        CurrentScreen::Main => match key.code {
            KeyCode::Char('n') => {
                app.current_screen = CurrentScreen::NewSshIdentity;
                true
            }
            KeyCode::Char('q') => {
                app.current_screen = CurrentScreen::Exiting;
                true
            }
            _ => false,
        },
        CurrentScreen::Exiting => match key.code {
            KeyCode::Char('y') => {
                app.exit = true;
                true
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main;
                true
            }
            _ => false,
        },
        CurrentScreen::NewSshIdentity => match key.code {
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main;
                true
            }
            _ => false,
        },
    };

    false
}
