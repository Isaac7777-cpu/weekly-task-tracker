use chrono::NaiveDate;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Commitment {
    pub id: i64,
    pub name: String,
    pub weekly_target_hours: f64,
    pub active: bool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CommitmentWithProgress {
    pub id: i64,
    pub name: String,
    pub weekly_target_hours: f64,
    pub current_week_total: Option<f64>,
    pub start_monday: NaiveDate,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct WeeklyStat {
    pub week_start: NaiveDate,
    pub total_hours: f64,
}
