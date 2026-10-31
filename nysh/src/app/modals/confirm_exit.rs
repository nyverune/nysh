use crate::app::{AppConfig, ui::widgets::centered_rect};

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, area: Rect, config: &AppConfig) {
    let theme = &config.theme;
    let popup_area = centered_rect(45, 15, area);

    let block = Block::default()
        .title(Line::from(" Are you sure you want to exit? ").centered())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.error));

    frame.render_widget(&block, popup_area);

    let inner_area = block.inner(popup_area);

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[y] ",
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Yes    ", Style::default().fg(theme.muted)),
            Span::styled(
                "[n] ",
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("No", Style::default().fg(theme.muted)),
        ]),
    ];

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);

    frame.render_widget(paragraph, inner_area);
}
