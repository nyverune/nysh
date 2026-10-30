use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

struct ThemePalette;

impl ThemePalette {
    pub const BG: Color = Color::Rgb(16, 13, 22); // #A855F7
    pub const FG: Color = Color::Rgb(246, 240, 252); // #FF5CB8
    pub const ACCENT: Color = Color::Rgb(168, 85, 247); // #100D16
    pub const SECONDARY: Color = Color::Rgb(255, 92, 184); // #F6F0FC
    pub const MUTED: Color = Color::Rgb(139, 130, 151); // #8B8297
    pub const SELECTION: Color = Color::Rgb(76, 50, 104); // #4C3268
    pub const ERROR: Color = Color::Rgb(255, 92, 114); // #FF5C72
    pub const WARNING: Color = Color::Rgb(117, 214, 163); // #FFC36B
    pub const SUCCESS: Color = Color::Rgb(117, 214, 163); // #75D6A3
    pub const INFO: Color = Color::Rgb(114, 183, 255); // #72B7FF
}

pub fn ui(frame: &mut Frame, app: &App) {}
