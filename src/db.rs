use std::path::PathBuf;

use crate::model::{Commitment, CommitmentWithProgress, WeeklyStat};
use chrono::{Datelike, Duration, Local, NaiveDate};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

fn db_path() -> PathBuf {
    let mut path = PathBuf::from("./data");
    std::fs::create_dir_all(&path).unwrap();
    path.push("weekly_commit.db");
    path
}

fn current_week_bounds() -> (NaiveDate, NaiveDate) {
    let today = Local::now().date_naive();
    let weekday = today.weekday().num_days_from_monday() as i64;

    let week_start = today - Duration::days(weekday);
    let next_week_start = week_start + Duration::days(7);

    (week_start, next_week_start)
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

pub async fn get_commitment(
    pool: &Pool<Sqlite>,
    id: i64,
) -> Result<Option<Commitment>, sqlx::Error> {
    let row = sqlx::query_as!(
        Commitment,
        r#"
        SELECT 
            id as "id!: i64", 
            name as "name!: String", 
            weekly_target_hours as "weekly_target_hours!: f64", 
            active as "active!: bool"
        FROM commitments
        WHERE id == ?1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
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

pub async fn current_week_progress_by_id(
    pool: &Pool<Sqlite>,
    commitment_id: i64,
) -> Result<Option<f64>, sqlx::Error> {
    let (start, end) = current_week_bounds();
    let start_str = start.to_string();
    let end_str = end.to_string();

    let total: Option<f64> = sqlx::query_scalar!(
        r#"
        SELECT SUM(pl.hours) as "total: f64"
        FROM progress_logs pl
        JOIN commitments c ON pl.commitment_id = c.id
        WHERE c.id = ?1
            AND c.active = 1
            AND pl.logged_at >= ?2
            AND pl.logged_at < ?3
        "#,
        commitment_id,
        start_str,
        end_str
    )
    .fetch_one(pool)
    .await?;

    Ok(total)
}

pub async fn list_commitments_with_week_progress(
    pool: &Pool<Sqlite>,
) -> Result<Vec<CommitmentWithProgress>, sqlx::Error> {
    let (start, end) = current_week_bounds();
    let start_str = start.to_string();
    let end_str = end.to_string();

    let rows = sqlx::query_as!(
        CommitmentWithProgress,
        r#"
        SELECT
            c.id as "id!: i64",
            c.name as "name!: String",
            c.weekly_target_hours as "weekly_target_hours!: f64",
            SUM(pl.hours) as "week_total: f64"
        FROM commitments c
        LEFT JOIN progress_logs pl
            ON pl.commitment_id = c.id
           AND pl.logged_at >= ?1
           AND pl.logged_at < ?2
        WHERE c.active = 1
        GROUP BY c.id, c.name, c.weekly_target_hours
        "#,
        start_str,
        end_str
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn weekly_stats_for_commitment(
    pool: &Pool<Sqlite>,
    commitment_id: i64,
) -> Result<Vec<WeeklyStat>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            date(pl.logged_at, 'weekday 1', '-7 days') as "week_start!: NaiveDate",
            SUM(pl.hours) as "total_hours!: f64"
        FROM progress_logs pl
        WHERE pl.commitment_id = ?1
        GROUP BY 1
        ORDER BY 1
        "#,
        commitment_id
    )
    .fetch_all(pool)
    .await?;

    let stats = rows
        .into_iter()
        .map(|r| WeeklyStat {
            _week_start: r.week_start,
            total_hours: r.total_hours,
        })
        .collect();

    Ok(stats)
}
