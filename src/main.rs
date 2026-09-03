mod models;
mod db;
mod cli;
mod service;

use db::{create_pool, init_db};
// use models::Note;
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = create_pool().await?;
    init_db(&pool).await?;
    println!("ok");
    db::add_note(&pool, "Hello", "World").await?;
    Ok(())
}
