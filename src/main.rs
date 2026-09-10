mod models;
mod db;
mod cli;
mod service;
mod handlers;
mod states;
mod error;

use clap::Parser;
use db::{create_pool, init_db};
// use models::Note;
use cli::Cli;
use sqlx::SqlitePool;
use error::AppError;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    let pool: SqlitePool = create_pool().await?;
    init_db(&pool).await?;

    let cli = Cli::parse();

    service::dispatch(cli, pool).await?;
    
    Ok(())
}
