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
    let selected_identity = app.identities.get(app.selected_index);

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
            Constraint::Length(9),
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

    // === Empty state ===
    if selected_identity.is_none() {
        let empty_state = Paragraph::new(format!("No SSH identities in {}", config.ssh_config))
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.muted));
        frame.render_widget(empty_state, chunks[2]);
        return;
    }

    let selected_identity = selected_identity.unwrap();

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

    let labels_and_values = vec![
        Line::from(vec![
            Span::styled("Type:        ", Style::default().fg(theme.muted)),
            Span::styled(
                selected_identity.key_type.to_string(),
                Style::default().fg(theme.fg),
            ),
        ]),
        Line::from(vec![
            Span::styled("Public key:  ", Style::default().fg(theme.muted)),
            Span::styled(
                selected_identity.public_key.clone(),
                Style::default().fg(theme.info),
            ),
        ]),
        Line::from(vec![
            Span::styled("Host alias:  ", Style::default().fg(theme.muted)),
            Span::styled(
                format!(
                    "{} -> {}",
                    selected_identity.alias, selected_identity.host_name
                ),
                Style::default().fg(theme.fg),
            ),
        ]),
        Line::from(vec![
            Span::styled("Fingerprint: ", Style::default().fg(theme.muted)),
            Span::styled(
                selected_identity.fingerprint.clone(),
                Style::default().fg(theme.muted),
            ),
        ]),
        Line::from(vec![
            Span::styled("Path:        ", Style::default().fg(theme.muted)),
            Span::styled(
                selected_identity.key_path.clone(),
                Style::default().fg(theme.info),
            ),
        ]),
        Line::from(vec![
            Span::styled("Status:      ", Style::default().fg(theme.muted)),
            Span::styled(
                format!("{}", selected_identity.auth_status),
                Style::default().fg(theme.success),
            ),
        ]),
    ];

    let details_paragraph = Paragraph::new(labels_and_values);
    frame.render_widget(details_paragraph, details);
}
