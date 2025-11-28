use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, CommitmentDisplayRecord};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
            // Constraint::Percentage(25),
        ])
        .split(f.area());

    draw_progress_pane(f, app, chunks[0]);
    draw_commitments_list_pane(f, app, chunks[1]);
}

fn draw_progress_pane(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Weekly Progress (Active)")
        .borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let active: Vec<&CommitmentDisplayRecord> = app.items.iter().filter(|c| c.0.active).collect();

    if active.is_empty() {
        let p = Paragraph::new("No active commitments.")
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: true });
        f.render_widget(p, inner);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            active
                .iter()
                .map(|_| Constraint::Length(3))
                .collect::<Vec<_>>(),
        )
        .split(inner);

    for (row_area, c) in rows.into_iter().zip(active.into_iter()) {
        let ratio = if c.0.weekly_target_hours <= 0.0 {
            0.0
        } else {
            (c.0.current_week_total.unwrap_or(0.0) / c.0.weekly_target_hours).min(1.5) // To have at least something       
        };

        let label = format!(
            "{} ({:.1}/{:.1}h)",
            c.0.name,
            c.0.current_week_total.unwrap_or(0.0),
            c.0.weekly_target_hours
        );

        let mut gauge_style = Style::default().fg(Color::Green);
        if ratio >= 1.0 {
            gauge_style = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
        }

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::NONE))
            .gauge_style(gauge_style)
            .ratio((ratio / 1.5).min(1.0))
            .label(Span::raw(label));

        f.render_widget(gauge, row_area.clone());
    }
}

fn draw_commitments_list_pane(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Commitments (j/k, gg/G, a, l, r)")
        .borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|c| {
            let marker = if c.0.active { "[A]" } else { "[ ]" };
            let line = format!(
                "{} #{:<3} {} (target {:.1}h)",
                marker, c.0.id, c.0.name, c.0.weekly_target_hours
            );
            let style = if c.0.active {
                Style::default()
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ListItem::new(Span::styled(line, style))
        })
        .collect();

    let list = List::new(items)
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, inner, &mut app.list_state.clone());
}

fn _draw_detail_pane(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().title("Detail").borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let Some(selected) = app.selected_item() else {
        let p =
            Paragraph::new("No commitment selected").style(Style::default().fg(Color::DarkGray));
        f.render_widget(p, inner);
        return;
    };

    let mut lines = Vec::new();
    lines.push(vec![Span::styled(
        format!("{} #{}", selected.0.name, selected.0.id),
        Style::default().add_modifier(Modifier::BOLD),
    )]);

    lines.push(vec![Span::from(format!(
        "Status: {}",
        if selected.0.active {
            "Active"
        } else {
            "Inactive"
        }
    ))]);
    // lines.push(Spans::from(format))
}
