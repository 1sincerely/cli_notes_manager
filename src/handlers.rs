use axum::{
    routing::get,
    Router,
};
use axum::http::StatusCode;
use axum::extract::{State, Json};
use crate::states::AppState;
use crate::db;
use crate::models::Note;

pub async fn hello_handler(State(state): State<AppState>) -> Result<Json<Vec<Note>>, axum::http::StatusCode>{
    let pool = state.state;
    let notes = db::list_notes(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(notes))
}

pub async fn create_router(state: AppState) -> Router{
    Router::new().route("/notes", get(hello_handler)).with_state(state)
}

pub async fn run_server(state: AppState) -> Result<(), axum::Error>{
    let app = create_router(state).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

