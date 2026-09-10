use axum::{
    routing::{get, post, put, delete},
    Router,
};
use axum::http::StatusCode;
use axum::extract::{State, Json, Path};
use crate::states::AppState;
use crate::db;
use crate::models::Note;

pub async fn hello_handler(State(state): State<AppState>) -> Result<Json<Vec<Note>>, axum::http::StatusCode>{
    let pool = state.state;
    let notes = db::list_notes(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(notes))
}
pub async fn get_note_handler(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<Note>, axum::http::StatusCode>{
    let pool = state.state;
    let note: Note = db::get_note(&pool, id).await.map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(note))
}

pub async fn delete_note_handler(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<u64>, axum::http::StatusCode> {
    let pool = state.state;
    let rows_affected = db::del_note(&pool, id).await.map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(rows_affected))
}

pub async fn add_note_handler(State(state): State<AppState>, Path((title, content)): Path<(String, String)>) -> Result<(), axum::http::StatusCode>{
    let pool = state.state;
    db::add_note(&pool, &title, &content).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
pub async fn update_note_handler(State(state): State<AppState>, Path((id, content)): Path<(i64, String)>) -> Result<Json<u64>, axum::http::StatusCode>{
    let pool = state.state;
    let res = db::update_note(&pool, id, content).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(res))
}

pub async fn create_router(state: AppState) -> Router{
    Router::new()
        .route("/notes", get(hello_handler))
        .route("/notes/:id", get(get_note_handler))
        .route("/notes/:id", delete(delete_note_handler))
        .route("/notes/add/:title/:content", post(add_note_handler))
        .route("/notes/:id/:context", put(update_note_handler))
        .with_state(state)
}

pub async fn run_server(state: AppState) -> Result<(), axum::Error>{
    let app = create_router(state).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

