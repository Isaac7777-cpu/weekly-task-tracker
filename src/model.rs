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
    pub week_total: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct WeeklyStat {
    pub _week_start: NaiveDate,
    pub total_hours: f64,
}
