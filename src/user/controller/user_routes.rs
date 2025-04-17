use crate::shared::app_state::AppState;

use super::user_handlers::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user))
        .route("/{id}", get(get_user_by_id))
        .route("/{id}", put(update_user))
        .route("/{id}", delete(delete_user))
}
