use crate::app::{App, AppConfig};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders},
};

struct LayoutAreas {
    top_content: Rect,
    bottom_content: Rect,
}

fn draw_layout(frame: &mut Frame, area: Rect, config: AppConfig) -> LayoutAreas {
    let theme = config.theme;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    // === Top ===
    let top = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.selection))
        .title_top(
            Line::from(format!(" {} ", config.app_name))
                .left_aligned()
                .style(Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)),
        )
        .title_top(
            Line::from(format!(" {} ", config.version))
                .right_aligned()
                .style(Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)),
        );

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

pub fn ui(frame: &mut Frame, app: &App, config: AppConfig) {
    draw_layout(frame, frame.area(), config);
}
