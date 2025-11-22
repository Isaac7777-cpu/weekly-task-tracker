#[derive(Debug, sqlx::FromRow)]
pub struct Commitment {
    // pub id: u32,
    pub id: i64,
    pub name: String,
    pub weekly_target_hours: f64,
    pub active: bool,
}
