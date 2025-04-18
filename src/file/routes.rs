use super::handlers::{delete_file, serve_protected_file};
use crate::shared::app_state::AppState;
use axum::{
    routing::{delete, get},
    Router,
};

pub fn file_routes() -> Router<AppState> {
    Router::new()
        .route("/{file_id}", get(serve_protected_file))
        .route("/{file_id}", delete(delete_file))
}
