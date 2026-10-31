use crate::app::{AppConfig, ui::layout::draw_layout};

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
};

pub fn draw(frame: &mut Frame, area: Rect, config: &AppConfig) {
    let theme = &config.theme;

    let keybinds = Line::from(vec![
        Span::styled(
            "[n] ",
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::styled("New    ", Style::default().fg(theme.muted)),
        Span::styled(
            "[q] ",
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Quit", Style::default().fg(theme.muted)),
    ]);

    draw_layout(frame, area, config, config.app_name.clone(), true, keybinds);
}
