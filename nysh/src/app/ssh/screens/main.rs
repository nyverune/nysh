use crate::app::{AppConfig, ui::layout::draw_layout};

use ratatui::{Frame, layout::Rect};

pub fn draw(frame: &mut Frame, area: Rect, config: &AppConfig) {
    draw_layout(frame, area, config, config.app_name.clone(), true);
}
