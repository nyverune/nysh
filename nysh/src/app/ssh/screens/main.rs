use crate::app::{
    App, AppConfig,
    ui::{
        layout::{draw_layout, draw_top_layout},
        widgets::separator,
    },
};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Padding, Paragraph},
};

pub fn draw(app: &App, frame: &mut Frame, area: Rect, config: &AppConfig) {
    let theme = &config.theme;
    let selected_identity = &app.identities[app.selected_index];

    let keybinds = Line::from(vec![
        Span::styled(
            "[n] ",
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::styled("New    ", Style::default().fg(theme.muted)),
        Span::styled(
            "[↑/↓] ",
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Navigate    ", Style::default().fg(theme.muted)),
        Span::styled(
            "[q] ",
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Quit", Style::default().fg(theme.muted)),
    ]);

    let inner_top = draw_layout(frame, area, config, config.app_name.clone(), true, keybinds);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(7),
        ])
        .split(inner_top);

    // === Draw title ===
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(10)])
        .split(chunks[0]);

    let title = Paragraph::new("SSH Identities")
        .style(Style::default().fg(theme.fg).add_modifier(Modifier::BOLD));
    frame.render_widget(title, header_chunks[0]);

    let count = Paragraph::new(format!("{} keys", app.identities.len()))
        .alignment(Alignment::Right)
        .style(Style::default().fg(theme.muted));
    frame.render_widget(count, header_chunks[1]);

    // === Separator ===
    frame.render_widget(separator(theme.selection), chunks[1]);

    // === List Identities ===
    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    let items: Vec<ListItem> = app
        .identities
        .iter()
        .enumerate()
        .map(|(i, identity)| {
            ListItem::new(format!(
                "{} {}",
                if i == app.selected_index { ">" } else { " " },
                identity.alias
            ))
        })
        .collect();

    let list = List::new(items)
        .style(Style::default().fg(theme.fg))
        .highlight_style(
            Style::default()
                .bg(theme.selection)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("");

    let mut state = ListState::default();
    state.select(Some(app.selected_index));

    frame.render_stateful_widget(list, list_chunks[0], &mut state);

    if app.identities.len() == 0 {
        return;
    }

    // === Separator ===
    frame.render_widget(separator(theme.selection), chunks[3]);

    // === Draw details ===
    let details = draw_top_layout(
        frame,
        chunks[4],
        config,
        format!("Details: {}", selected_identity.alias),
        false,
        Padding::new(1, 1, 1, 0),
    );

    let detail_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(15), Constraint::Min(1)])
        .split(details);

    let labels = vec![
        Line::from("Type:"),
        Line::from("Public key:"),
        Line::from("Host alias:"),
        Line::from("Fingerprint:"),
    ];

    let label_paragraph = Paragraph::new(labels);
    frame.render_widget(label_paragraph, detail_chunks[0]);

    let keys = vec![
        Line::from(selected_identity.key_type.to_string()),
        Line::from(selected_identity.public_key.clone()),
        Line::from(format!("{} -> {}", selected_identity.alias, "github.com")),
        Line::from(selected_identity.fingerprint.clone()),
    ];

    let keys_paragraph = Paragraph::new(keys);
    frame.render_widget(keys_paragraph, detail_chunks[1]);
}
