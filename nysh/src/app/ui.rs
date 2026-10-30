use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders},
};

use crate::app::App;

struct ThemePalette;

impl ThemePalette {
    pub const BG: Color = Color::Rgb(16, 13, 22); // #100D16
    pub const FG: Color = Color::Rgb(246, 240, 252); // #F6F0FC
    pub const ACCENT: Color = Color::Rgb(168, 85, 247); // #A855F7
    pub const SECONDARY: Color = Color::Rgb(255, 92, 184); // #FF5CB8
    pub const MUTED: Color = Color::Rgb(139, 130, 151); // #8B8297
    pub const SELECTION: Color = Color::Rgb(76, 50, 104); // #4C3268
    pub const ERROR: Color = Color::Rgb(255, 92, 114); // #FF5C72
    pub const WARNING: Color = Color::Rgb(255, 195, 107); // #FFC36B
    pub const SUCCESS: Color = Color::Rgb(117, 214, 163); // #75D6A3
    pub const INFO: Color = Color::Rgb(114, 183, 255); // #72B7FF
}

const VERSION: &str = "0.0.0";

struct LayoutAreas {
    top_content: Rect,
    bottom_content: Rect,
}

fn draw_layout(frame: &mut Frame, area: Rect) -> LayoutAreas {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    // === Top ===
    let top = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ThemePalette::SELECTION))
        .title_top(
            Line::from(" Nysh ").left_aligned().style(
                Style::default()
                    .fg(ThemePalette::FG)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .title_top(
            Line::from(format!(" {} ", VERSION)).right_aligned().style(
                Style::default()
                    .fg(ThemePalette::FG)
                    .add_modifier(Modifier::BOLD),
            ),
        );

    frame.render_widget(&top, chunks[0]);

    // === Bottom ===
    let bottom = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ThemePalette::SELECTION));

    frame.render_widget(&bottom, chunks[1]);

    // === Return ===
    LayoutAreas {
        top_content: top.inner(chunks[0]),
        bottom_content: bottom.inner(chunks[1]),
    }
}

pub fn ui(frame: &mut Frame, app: &App) {
    draw_layout(frame, frame.area());
}
