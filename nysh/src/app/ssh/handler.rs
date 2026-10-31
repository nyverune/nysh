use crate::app::{App, CurrentModal, CurrentScreen};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn handler(app: &mut App, key: KeyEvent) -> bool {
    if app.current_screen == CurrentScreen::Main && app.current_modal == CurrentModal::ConfirmExit {
        return match key.code {
            KeyCode::Char('y') => {
                app.exit = true;
                true
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                app.current_modal = CurrentModal::NoModal;
                true
            }
            _ => true,
        };
    }

    match app.current_screen {
        CurrentScreen::Main => match key.code {
            KeyCode::Char('n') => {
                app.current_screen = CurrentScreen::NewSshIdentity;
                true
            }
            KeyCode::Char('q') => {
                app.current_modal = CurrentModal::ConfirmExit;
                true
            }
            _ => false,
        },
        CurrentScreen::NewSshIdentity => match key.code {
            KeyCode::Char('c') => {
                // TODO: Handle create
                true
            }
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main;
                true
            }
            _ => false,
        },
    };

    false
}
