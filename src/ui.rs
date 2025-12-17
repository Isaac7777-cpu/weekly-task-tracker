use std::collections::HashMap;

use chrono::{Duration, NaiveDate};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize, palette::tailwind},
    text::{Line, Span},
    widgets::{
        Bar, BarChart, BarGroup, Block, BorderType, Borders, Clear, Gauge, List, ListItem,
        Paragraph, Wrap,
    },
};

use crate::{
    app::{App, CommitmentDisplayRecord, InputMode},
    model::WeeklyStat,
    util::get_monday_this_week,
};

struct HistorySummary {
    pub start_monday: NaiveDate,
    pub weeks_passed: i64,
    pub total_required: f64,
    pub total_done: f64,
    pub delta: f64,
}

fn draw_horizontal_separator(f: &mut Frame, area: Rect, spacer: Rect, sep_char: char) {
    let separator_area = Rect {
        x: area.x,
        y: spacer.y,
        width: area.width,
        height: 1,
    };

    f.render_widget(
        Paragraph::new(sep_char.to_string().repeat(area.width as usize))
            .wrap(Wrap { trim: false })
            .style(
                Style::default()
                    .fg(tailwind::GRAY.c600)
                    .add_modifier(Modifier::BOLD),
            ),
        separator_area,
    );
}

fn draw_vertical_separator(f: &mut Frame, area: Rect, spacer: Rect, sep_char: char) {
    let separator_area = Rect {
        x: spacer.x,
        y: area.y,
        width: 1,
        height: area.height,
    };

    f.render_widget(
        Paragraph::new(sep_char.to_string().repeat(area.height as usize))
            .wrap(Wrap { trim: false })
            .style(
                Style::default()
                    .fg(tailwind::GRAY.c600)
                    .add_modifier(Modifier::BOLD),
            ),
        separator_area,
    );
}

fn compute_history_summary(
    start_monday: NaiveDate,
    weekly_target_hours: f64,
    stats: &[WeeklyStat],
) -> HistorySummary {
    let this_monday = get_monday_this_week();
    let weeks_passed = ((this_monday - start_monday).num_weeks() + 1).max(0);

    let total_done: f64 = stats.iter().map(|s| s.total_hours).sum();
    let total_required = weekly_target_hours * weeks_passed as f64;
    let delta = total_done - total_required;

    HistorySummary {
        start_monday,
        weeks_passed,
        total_required,
        total_done,
        delta,
    }
}

fn render_commitment_history_summary_as_paragraph(summary: HistorySummary) -> Paragraph<'static> {
    let status_text = if summary.delta < -1e-6 {
        format!("Due by {:.1} h", -summary.delta + 0.0)
    } else if summary.delta > 1e-6 {
        format!("Overdone by {:.1} h", summary.delta + 0.0)
    } else {
        "On track".to_string()
    };
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Start Monday: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(summary.start_monday.format("%Y-%m-%d").to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Weeks passed: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(summary.weeks_passed.to_string()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Weekly Target:  ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "{:.1} h",
                summary.total_required / (summary.weeks_passed as f64) + 0.0
            )),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Required:        ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{:.1} h", summary.total_required)),
        ]),
        Line::from(vec![
            Span::styled(
                "Done:            ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{:.1} h", summary.total_done + 0.0)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Accumulated Status: ",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(if summary.delta < 0.0 {
                        Color::Red
                    } else if summary.delta > 0.0 {
                        Color::Green
                    } else {
                        Color::Gray
                    }),
            ),
            Span::raw(status_text),
        ]),
        Line::from(""),
    ];
    Paragraph::new(lines).wrap(Wrap { trim: false }).block(
        Block::default()
            .borders(Borders::ALL)
            .title_bottom("Details for reminder")
            .title_style(Modifier::UNDERLINED),
    )
}

fn popup_area(area: Rect, constraint_x: Constraint, constraint_y: Constraint) -> Rect {
    let vertical = Layout::vertical([constraint_y]).flex(Flex::Center);
    let horizontal = Layout::horizontal([constraint_x]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Percentage(15),
            Constraint::Length(1),
        ])
        .split(f.area());
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[0]);

    draw_progress_pane(f, app, panes[0]);
    draw_commitments_list_pane(f, app, panes[1]);
    draw_detail_pane(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);

    // TODO: Draw other editing related screen
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::LogHours => draw_log_overlay(f, app),

        // TODO: Implement the UI
        InputMode::CreateCommitment => draw_create_commitment_overlay(f, app),
    }
}

fn draw_log_overlay(f: &mut Frame, app: &mut App) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title_bottom("Log Hours");
    let area = popup_area(
        f.area(),
        Constraint::Percentage(40),
        Constraint::Length(22),
    );
    let inner = block.inner(area);

    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let Some(item) = app.get_selected_item() else {
        f.render_widget(Paragraph::new("No Selected Item..."), inner);
        return;
    };

    let (chunks, spacers) = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(13),
        Constraint::Length(3),
    ])
    .horizontal_margin(1)
    .spacing(2)
    .split_with_spacers(inner);

    // Draw the title
    f.render_widget(
        Span::styled(
            format!("Logging for \"{}\" (#{})", item.0.name, item.0.id),
            Style::default()
                .bold()
                .underlined()
                .bg(tailwind::STONE.c900)
                .fg(tailwind::ROSE.c500),
        ),
        chunks[0],
    );

    // Draw a separator
    draw_horizontal_separator(f, inner, spacers[1], '-');

    // Display the details of the commitment
    let summary = compute_history_summary(item.0.start_monday, item.0.weekly_target_hours, &item.1);
    let details_widget = render_commitment_history_summary_as_paragraph(summary);
    f.render_widget(details_widget, chunks[1]);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                app.input_buffer.clone(),
                Style::default().bold().underlined(),
            ),
            Span::styled(" ◁", Style::default().bold()),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Amount:")
                .border_type(BorderType::Rounded),
        ),
        chunks[2],
    );
}

fn draw_create_commitment_overlay(f: &mut Frame, app: &mut App) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title_bottom("Create Commitment");
    let area = popup_area(
        f.area(),
        Constraint::Percentage(40),
        Constraint::Length(39),
    );
    let _inner = block.inner(area);

    f.render_widget(Clear, area);
    f.render_widget(block, area);
}

fn draw_progress_pane(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Weekly Progress (Active)")
        .borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let active: Vec<&CommitmentDisplayRecord> =
        app.get_items().iter().filter(|c| c.0.active).collect();

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

        let mut gauge_style = Style::default().fg(tailwind::GREEN.c700);
        if ratio >= 1.0 {
            gauge_style = Style::default()
                .fg(tailwind::CYAN.c600)
                .add_modifier(Modifier::BOLD);
        }

        let label = format!(
            "{} ({:.1}/{:.1}h)",
            c.0.name,
            c.0.current_week_total.unwrap_or(0.0),
            c.0.weekly_target_hours
        );
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
            .gauge_style(gauge_style)
            .ratio((ratio / 1.5).min(1.0).max(0.005))
            .label(Span::raw(format!("{} %", (ratio * 100.0).round())))
            .use_unicode(true);

        f.render_widget(gauge, *row_area);

        let lable_width = label.chars().count() as u16;
        let label_area = Rect {
            x: row_area.x + 2,
            y: row_area.y,
            width: lable_width.min((row_area.width as f32 * 0.3).ceil() as u16),
            height: 1,
        };
        let label_widget = Paragraph::new(label).style(Style::default().bg(Color::Reset));

        f.render_widget(label_widget, label_area);
    }
}

fn draw_commitments_list_pane(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title("Commitments (j/k, gg/G, a, l, r)")
        .borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let items: Vec<ListItem> = app
        .get_items()
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
                .bg(tailwind::BLUE.c500)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, inner, &mut app.list_state);
}

fn draw_detail_pane(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().title("Detail").borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let (chunks, spacers) = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(3), Constraint::Fill(2)])
        .spacing(2)
        .split_with_spacers(inner);

    let Some(selected) = app.get_selected_item() else {
        let p =
            Paragraph::new("No commitment selected").style(Style::default().fg(Color::DarkGray));
        f.render_widget(p, inner);
        return;
    };

    let weekly_stats = &selected.1;

    // Prepare for checking historic hours
    let start_monday: NaiveDate = selected.0.start_monday;
    let end_monday: NaiveDate = get_monday_this_week();

    let mut weeks: Vec<NaiveDate> = Vec::new();
    let mut current = start_monday;
    while current <= end_monday {
        weeks.push(current);
        current += Duration::weeks(1);
    }

    let hours_by_week: HashMap<NaiveDate, f64> = weekly_stats
        .iter()
        .map(|s| (s.week_start, s.total_hours))
        .collect();

    // Prepare bar chart entries
    let bars: Vec<Bar> = weeks
        .iter()
        .enumerate()
        .map(|(i, week)| {
            let hours = hours_by_week.get(week).copied().unwrap_or(0.0);

            let bar_style = Style::default().fg(tailwind::ROSE.c500);

            Bar::default()
                .value(hours.round() as u64)
                .style(bar_style)
                .label(Line::from(format!("W{}", i + 1)))
                .text_value(format!("{}h", hours.round()))
        })
        .collect();

    let max = (selected.0.weekly_target_hours * 1.5).ceil() as u64;

    let chart = BarChart::default()
        .block(Block::default().title("Weekly Hours"))
        .data(BarGroup::default().bars(&bars))
        .bar_width(4)
        .bar_gap(1)
        .max(max);

    f.render_widget(chart, chunks[0]);

    // Draw the summary
    draw_history_summary(f, app, chunks[1]);

    // Draw the separator
    draw_vertical_separator(f, inner, spacers[1], '|');
}

fn draw_history_summary(f: &mut Frame, app: &App, area: Rect) {
    let Some(selected) = app.get_selected_item() else {
        // nothing selected – draw placeholder
        let placeholder = Paragraph::new("No commitment selected").wrap(Wrap { trim: true });
        f.render_widget(placeholder, area);
        return;
    };

    // however you store them:
    let commitment: &CommitmentDisplayRecord = selected;

    // get the stats for this commitment (you may already have them cached in App)
    let summary = compute_history_summary(
        commitment.0.start_monday,
        commitment.0.weekly_target_hours,
        &commitment.1,
    );

    let status_text = if summary.delta < -1e-6 {
        format!("Due by {:.1} h", -summary.delta + 0.0)
    } else if summary.delta > 1e-6 {
        format!("Overdone by {:.1} h", summary.delta + 0.0)
    } else {
        "On track".to_string()
    };

    let lines = vec![
        Line::from(vec![
            Span::styled(
                "Start Monday: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(summary.start_monday.format("%Y-%m-%d").to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Weeks passed: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(summary.weeks_passed.to_string()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Required: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{:.1} h", summary.total_required)),
        ]),
        Line::from(vec![
            Span::styled("Done:     ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{:.1} h", summary.total_done + 0.0)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Accumulated Status: ",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(if summary.delta < 0.0 {
                        Color::Red
                    } else if summary.delta > 0.0 {
                        Color::Green
                    } else {
                        Color::Gray
                    }),
            ),
            Span::raw(status_text),
        ]),
    ];

    let widget = Paragraph::new(lines).wrap(Wrap { trim: true });

    f.render_widget(widget, area);
}

impl InputMode {
    fn get_style(&self) -> Style {
        match self {
            InputMode::Normal => Style::default().bg(tailwind::PURPLE.c800),
            InputMode::LogHours => Style::default().bg(tailwind::CYAN.c800),
            InputMode::CreateCommitment => Style::default().bg(tailwind::EMERALD.c800),
        }
    }
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Max(12),
            Constraint::Fill(2),
            Constraint::Fill(3),
        ])
        .split(area);

    f.render_widget(
        Span::styled(
            format!(" {} ", app.input_mode.to_string()),
            app.input_mode.get_style(),
        ),
        chunks[0],
    );

    f.render_widget(
        Paragraph::new(app.get_quick_msg())
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(tailwind::RED.c300)),
        chunks[1],
    );

    f.render_widget(
        Paragraph::new(app.get_keymap_msg())
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(tailwind::GRAY.c100)),
        chunks[2],
    );
}
