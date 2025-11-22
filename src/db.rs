use std::path::PathBuf;

use chrono::{Datelike, Duration, Local};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

// use dirs;

#[derive(Debug)]
pub struct Commitment {
    // pub id: u32,
    pub id: i64,
    pub name: String,
    pub weekly_target_hours: f64,
    pub active: bool,
}

fn db_path() -> PathBuf {
    let mut path = PathBuf::from("./data");
    std::fs::create_dir_all(&path).unwrap();
    path.push("weekly_commit.db");
    path
}

pub async fn open_db() -> Pool<Sqlite> {
    let path = db_path().to_string_lossy().to_string();
    let url = format!("sqlite://{}", path);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to open DB")
}

pub async fn add_commitment(
    pool: &Pool<Sqlite>,
    name: &str,
    weekly_hours: f64,
) -> Result<i64, sqlx::Error> {
    let today = Local::now().date_naive();
    let today_str = today.to_string();
    let weekday = today.weekday().num_days_from_monday() as i64;

    // Monday of upcoming week
    let week_start = today - Duration::days(weekday - 7);
    let week_start_str = week_start.to_string();

    let row = sqlx::query!(
        r#"
        INSERT INTO commitments (name, weekly_target_hours, active, created_at, start_week_monday) 
        VALUES (?1, ?2, 1, ?3, ?4)
        RETURNING id;
        "#,
        name,
        weekly_hours,
        today_str,
        week_start_str
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn archive_commiment(pool: &Pool<Sqlite>, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE commitments
        SET active = 0
        WHERE id = ?1 AND active = 1;
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn reactivate_commiment(pool: &Pool<Sqlite>, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE commitments
        SET active = 1
        WHERE id = ?1 AND active = 0;
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn list_commitments(pool: &Pool<Sqlite>) -> Vec<Commitment> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, weekly_target_hours, active
        FROM commitments
        WHERE active = 1
        ORDER BY id;
        "#
    )
    .fetch_all(pool)
    .await
    .expect("Query failed");

    rows.into_iter()
        .map(|r| Commitment {
            id: r.id,
            name: r.name,
            weekly_target_hours: r.weekly_target_hours,
            active: r.active != 0,
        })
        .collect()
}

pub async fn log_record(pool: &Pool<Sqlite>, name: &str, hours: f32) -> Result<i64, sqlx::Error> {
    let log_time = Local::now().date_naive().to_string();

    let row = sqlx::query!(
        r#"
        INSERT INTO progress_logs (commitment_id, hours, logged_at) 
        SELECT id, ?2, ?3
        FROM commitments
        WHERE name = ?1 AND active = 1
        RETURNING id;
        "#,
        name,
        hours,
        log_time
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn log_record_id(
    pool: &Pool<Sqlite>,
    commitment_id: i64,
    hours: f32,
) -> Result<i64, sqlx::Error> {
    let log_time = Local::now().date_naive().to_string();

    let row = sqlx::query!(
        r#"
        INSERT INTO progress_logs (commitment_id, hours, logged_at) 
        VALUES (?1, ?2, ?3)
        RETURNING id;
        "#,
        commitment_id,
        hours,
        log_time
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
