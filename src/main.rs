mod models;
mod db;
mod cli;
mod service;

use clap::Parser;
use db::{create_pool, init_db, add_note};
// use models::Note;
use cli::{Cli, Commands};
use models::Note;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = create_pool().await?;
    init_db(&pool).await?;
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { title, content} => {
            add_note(&pool, &title, &content).await?;
        }
        Commands::List => {
            let notes: Vec<Note> = db::list_notes(&pool).await?;
            service::show_notes(&notes).await;
        }
        Commands::Remove { id } => { 
            let rows: u64 = db::del_note(&pool, &id).await?;
            service::remove_handle(&rows).await;
        }
    }

    Ok(())
}
