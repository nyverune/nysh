mod layout;
pub mod widgets;

use crate::app::{App, AppConfig, CurrentScreen, ssh::screens};
use layout::draw_layout;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::Block,
};

pub fn draw(frame: &mut Frame, app: &App, config: &AppConfig) {
    let area = frame.area();

    frame.render_widget(
        Block::default().style(Style::default().bg(config.theme.bg)),
        area,
    );

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area)[1];

    let inner_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(inner_area)[1];

    match app.current_screen {
        CurrentScreen::Main => {
            draw_layout(frame, inner_area, config, "Nysh".to_string(), true);
        }
        CurrentScreen::Exiting => {
            screens::exit::draw(frame, inner_area, config);
        }
        CurrentScreen::NewSshIdentity => {
            draw_layout(
                frame,
                inner_area,
                config,
                "New SSH Identity".to_string(),
                false,
            );
        }
    }
}
