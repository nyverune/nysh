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
            KeyCode::Down => {
                if !app.identities.is_empty() {
                    if app.selected_index >= app.identities.len().saturating_sub(1) {
                        app.selected_index = 0;
                    } else {
                        app.selected_index += 1;
                    }
                }
                true
            }
            KeyCode::Up => {
                if !app.identities.is_empty() {
                    if app.selected_index == 0 {
                        app.selected_index = app.identities.len().saturating_sub(1);
                    } else {
                        app.selected_index -= 1;
                    }
                }
                true
            }
            KeyCode::Char('c') => {
                if !app.identities.is_empty() {
                    let text = app
                        .identities
                        .get(app.selected_index)
                        .unwrap()
                        .public_key
                        .clone();

                    let _ = app.clipboard.set_text(text);
                }
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
