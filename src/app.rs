use core::fmt;
use ratatui::widgets::ListState;
use sqlx::SqlitePool;
use std::time::Instant;

use crate::{
    db::{list_all_commitments_with_week_progress, weekly_stats_for_commitment},
    model::{CommitmentWithProgress, WeeklyStat},
};

pub type CommitmentDisplayRecord = (CommitmentWithProgress, Vec<WeeklyStat>);

#[derive(Debug, Clone)]
pub enum InputMode {
    Normal,
    LogHours,
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputMode::Normal => write!(f, "NORMAL"),
            InputMode::LogHours => write!(f, "LOG HOUR"),
        }
    }
}

const NORMAL_MSG: &str =
    "q: quit | j/k: move | c: add commitment | l: log | r: reactivate | a: archive";
const LOG_MSG: &str = "q/ESC: quit | numeric characters (0-9, .): Hours Input";

pub struct App {
    pool: SqlitePool,
    items: Vec<CommitmentDisplayRecord>,
    pub input_buffer: String,
    pub list_state: ListState,
    pub input_mode: InputMode,
    pub message: String,
    pub last_refresh: Instant,
}

impl App {
    pub async fn new(pool: SqlitePool) -> anyhow::Result<Self> {
        let mut app = Self {
            pool: pool,
            items: Vec::new(),
            list_state: ListState::default(),
            input_mode: InputMode::Normal,
            // input_buffer: String::new(),
            input_buffer: "Test".to_string(),
            message: String::from(NORMAL_MSG),
            last_refresh: Instant::now(),
        };
        app.refresh_from_db().await?;
        if !app.items.is_empty() {
            app.list_state.select(Some(0));
        }
        Ok(app)
    }

    pub async fn refresh_from_db(&mut self) -> anyhow::Result<()> {
        let commitments_with_progs = list_all_commitments_with_week_progress(&self.pool).await?;

        self.items.clear();
        for c_pg in commitments_with_progs {
            let stats = weekly_stats_for_commitment(&self.pool, c_pg.id).await?;

            self.items.push((c_pg, stats));
        }
        self.items.sort_by_key(|c| (!c.0.active, c.0.id));

        self.last_refresh = Instant::now();

        Ok(())
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.list_state.selected()
    }

    pub fn set_message<S: Into<String>>(&mut self, msg: S) {
        self.message = msg.into();
    }

    pub fn selected_item(&self) -> Option<&CommitmentDisplayRecord> {
        self.selected_index().and_then(|idx| self.items.get(idx))
    }

    pub fn get_items(&self) -> &[CommitmentDisplayRecord] {
        &self.items
    }

    pub fn next(&mut self) {
        let i = match self.selected_index() {
            Some(i) if !self.items.is_empty() => (i + 1) % self.items.len(),
            _ => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let len = self.items.len();
        if len == 0 {
            self.list_state.select(None);
            return;
        }
        let i = match self.selected_index() {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        };
        self.list_state.select(Some(i));
    }

    pub fn jump_first(&mut self) {
        if !self.items.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn jump_last(&mut self) {
        if !self.items.is_empty() {
            self.list_state.select(Some(self.items.len() - 1));
        }
    }

    pub async fn reactivate_selected(&mut self) -> anyhow::Result<()> {
        if let Some(sel) = self.selected_item() {
            if !sel.0.active {
                crate::db::reactivate_commiment(&self.pool, sel.0.id).await?;
                self.set_message(format!("Reactivated #{}", sel.0.id));
                self.refresh_from_db().await?;
            };
        };
        Ok(())
    }

    pub async fn archive_selected(&mut self) -> anyhow::Result<()> {
        if let Some(sel) = self.selected_item() {
            if sel.0.active {
                crate::db::archive_commiment(&self.pool, sel.0.id).await?;
                self.set_message(format!("Archived #{}", sel.0.id));
                self.refresh_from_db().await?;
            };
        };
        Ok(())
    }

    pub fn get_message(state: &InputMode) -> String {
        match state {
            InputMode::Normal => NORMAL_MSG.to_string(),
            InputMode::LogHours => LOG_MSG.to_string(),
        }
    }

    pub fn switch_state(&mut self, target_state: InputMode) {
        self.message = App::get_message(&target_state);
        match target_state {
            InputMode::LogHours => {
                self.input_buffer = String::new();
            }
            _ => {}
        }
        self.input_mode = target_state;
    }
}
