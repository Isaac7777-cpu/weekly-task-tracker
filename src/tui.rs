use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use sqlx::SqlitePool;

use crate::app::{App, InputMode};

pub async fn run_tui(pool: SqlitePool) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(pool).await?;
    let tick_rate = Duration::from_millis(20);
    let mut last_tick = Instant::now();

    let res = loop {
        terminal.draw(|f| crate::ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if handle_key_event(key, &mut app).await? {
                    break Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

async fn handle_key_event(key: event::KeyEvent, app: &mut App) -> anyhow::Result<bool> {
    // TODO: Make the TUI apps editable
    match app.input_mode {
        InputMode::Normal => handle_normal_mode(key, app).await,
        InputMode::LogHours => handle_log_hour_mode(key, app).await,
        // _ => Ok(false),
    }
}

async fn handle_normal_mode(key: event::KeyEvent, app: &mut App) -> anyhow::Result<bool> {
    // TODO: Make the operations on the UI be non-blocking
    match key.code {
        KeyCode::Char('q') => {
            return Ok(true);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.jump_first();
        }
        KeyCode::Char('G') => {
            app.jump_last();
        }
        KeyCode::Char('r') => {
            app.reactivate_selected().await?;
        }
        KeyCode::Char('a') => {
            app.archive_selected().await?;
        }
        KeyCode::Char('l') => {
            if let Some(sel) = app.selected_item() {
                if sel.0.active {
                    app.input_mode = InputMode::LogHours;
                } else {
                    app.set_message("You can only log hours for activated items");
                }
            }
        }
        KeyCode::Char('c') => {
            app.set_message("New commitment name: (Enter to confirm, ESC to cancel)");
        }
        _ => {}
    };

    Ok(false)
}

async fn handle_log_hour_mode(key: event::KeyEvent, app: &mut App) -> anyhow::Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }

    return Ok(false);
}
