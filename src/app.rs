use ratatui::widgets::ListState;
use sqlx::SqlitePool;
use std::time::Instant;

use crate::{
    db::{list_commitments_with_week_progress, weekly_stats_for_commitment},
    model::{CommitmentWithProgress, WeeklyStat},
};

pub type CommitmentDisplayRecord = (CommitmentWithProgress, Vec<WeeklyStat>);

#[derive(Debug, Clone)]
pub enum InputMode {
    Normal,
    LogHours,
}

pub struct App {
    pub items: Vec<CommitmentDisplayRecord>,
    pub list_state: ListState,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub message: String,
    pub last_refresh: Instant,
}

impl App {
    pub async fn new(pool: &SqlitePool) -> anyhow::Result<Self> {
        let mut app = Self {
            items: Vec::new(),
            list_state: ListState::default(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            message: String::from("q: quit | j/k: move | a: add | l: log | r: reactivate"),
            last_refresh: Instant::now(),
        };
        app.refresh_from_db(pool).await?;
        if !app.items.is_empty() {
            app.list_state.select(Some(0));
        }
        Ok(app)
    }

    pub async fn refresh_from_db(&mut self, pool: &SqlitePool) -> anyhow::Result<()> {
        let commitments_with_progs = list_commitments_with_week_progress(pool).await?;

        self.items.clear();
        for c_pg in commitments_with_progs {
            let stats = weekly_stats_for_commitment(pool, c_pg.id).await?;

            self.items.push((c_pg, stats));
        }

        self.last_refresh = Instant::now();

        Ok(())
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.list_state.selected()
    }

    pub fn selected_item(&self) -> Option<&CommitmentDisplayRecord> {
        self.selected_index().and_then(|idx| self.items.get(idx))
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

    pub fn set_message<S: Into<String>>(&mut self, msg: S) {
        self.message = msg.into();
    }
}
