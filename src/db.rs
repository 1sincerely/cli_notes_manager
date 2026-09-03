use dotenvy::dotenv;
use std::env;
use sqlx::{pool::maybe::MaybePoolConnection::PoolConnection, sqlite::{SqlitePool, SqlitePoolOptions}};

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("database url must be set");

    SqlitePoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )
        "#
    )
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_note(pool: &SqlitePool, title: &str, content: &str) -> Result<(), sqlx::Error>{
    sqlx::query(
        r#"
        INSERT INTO notes (title, content, created_at, updated_at)
        VALUES ($1, $2, datetime('now'), datetime('now'))
        "#
    )
        .bind(title)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(())
}