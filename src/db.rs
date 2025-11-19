use std::path::PathBuf;

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
    let row = sqlx::query!(
        r#"
        INSERT INTO commitments (name, weekly_target_hours, active) 
        VALUES (?1, ?2, 1)
        RETURNING id;
        "#,
        name,
        weekly_hours
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
