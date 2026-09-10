use crate::models::Note;
use crate::states::AppState;
use sqlx::SqlitePool;
use crate::cli::{Mode, Commands, Cli};
use crate::handlers::run_server;
use crate::db;
use crate::error::AppError;


pub async fn run_cli(cli: Cli, pool: SqlitePool) -> Result<(), AppError>{
    match cli.command {
        Some(Commands::Add { title, content}) => {
            db::add_note(&pool, &title, &content).await?;
        }
        Some(Commands::List) => {
            let notes: Vec<Note> = db::list_notes(&pool).await?;
            show_notes(&notes).await;
        }
        Some(Commands::Remove { id }) => { 
            let rows: u64 = db::del_note(&pool, id).await?;
            remove_handle(&rows).await;
        }
        None => {
            println!("Enter a command please!");
        }
    }

    Ok(())
}

pub async fn run_web(pool: SqlitePool) -> Result<(), AppError>{
    let state = AppState {state: pool};
    run_server(state).await?;

    Ok(())
}
pub async fn show_notes(vec: &Vec<Note>){
    for note in vec {
        println!("{} | {} | {} | {} | {}", note.id, note.title, note.content, note.created_at, note.updated_at);
    }
}

pub async fn remove_handle(rows_affected: &u64) {
    if *rows_affected == 0 {
        println!("can't find note with that id!");
    } else {
        println!("{} rows affected!", rows_affected);
    }
}

pub async fn dispatch(cli: Cli, pool: SqlitePool) -> Result<(), AppError>{
    match cli.mode {
        Mode::Cli => {
            run_cli(cli, pool).await?;
        }
        Mode::Web => {
            run_web(pool).await?;
        }
    }

    Ok(())
}