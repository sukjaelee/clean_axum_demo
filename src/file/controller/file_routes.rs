use crate::shared::app_state::AppState;

use crate::file::controller::file_handler::{delete_file, serve_protected_file};
use axum::{
    routing::{delete, get},
    Router,
};

pub fn file_routes() -> Router<AppState> {
    Router::new()
        .route("/{file_id}", get(serve_protected_file))
        .route("/{file_id}", delete(delete_file))
}
