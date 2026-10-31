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
            "[c] ",
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Create    ", Style::default().fg(theme.muted)),
        Span::styled(
            "[esc] ",
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Cancel", Style::default().fg(theme.muted)),
    ]);

    draw_layout(
        frame,
        area,
        config,
        "New SSH Identity".to_string(),
        true,
        keybinds,
    );
}
