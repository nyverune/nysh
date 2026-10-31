use crate::app::{App, AppConfig, CurrentScreen};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

struct LayoutAreas {
    top_content: Rect,
    bottom_content: Rect,
}

fn draw_layout(
    frame: &mut Frame,
    area: Rect,
    config: &AppConfig,
    title: String,
    show_version: bool,
) -> LayoutAreas {
    let theme = &config.theme;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    // === Top ===
    let mut top = Block::default()
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
                .style(Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)),
        );
    }

    frame.render_widget(&top, chunks[0]);

    // === Bottom ===
    let bottom = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.selection));

    frame.render_widget(&bottom, chunks[1]);

    // === Return ===
    LayoutAreas {
        top_content: top.inner(chunks[0]),
        bottom_content: bottom.inner(chunks[1]),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_exiting_ui(frame: &mut Frame, area: Rect, config: &AppConfig) {
    let theme = &config.theme;
    let popup_area = centered_rect(45, 20, area);

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

pub fn ui(frame: &mut Frame, app: &App, config: &AppConfig) {
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
            draw_exiting_ui(frame, inner_area, config);
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
