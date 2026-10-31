use crate::app::AppConfig;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph},
};

pub fn draw_top_layout(
    frame: &mut Frame,
    area: Rect,
    config: &AppConfig,
    title: String,
    show_version: bool,
    padding: Padding,
) -> Rect {
    let theme = &config.theme;

    let mut top = Block::default()
        .padding(padding)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.selection))
        .title_top(
            Line::from(format!(" {} ", title))
                .left_aligned()
                .style(Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)),
        );

    if show_version {
        top = top.title_top(
            Line::from(format!(" {} ", config.version))
                .right_aligned()
                .style(
                    Style::default()
                        .fg(theme.muted)
                        .add_modifier(Modifier::BOLD),
                ),
        );
    }

    frame.render_widget(&top, area);

    top.inner(area)
}

pub fn draw_layout(
    frame: &mut Frame,
    area: Rect,
    config: &AppConfig,
    title: String,
    show_version: bool,
    keybinds: Line<'static>,
) -> Rect {
    let theme = &config.theme;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    // === Top ===

    let top = draw_top_layout(
        frame,
        chunks[0],
        config,
        title,
        show_version,
        Padding::new(2, 2, 1, 1),
    );

    // === Bottom ===
    let bottom = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.selection));

    let paragraph = Paragraph::new(keybinds)
        .alignment(Alignment::Center)
        .block(bottom);

    frame.render_widget(paragraph, chunks[1]);

    // === Return ===
    top
}
