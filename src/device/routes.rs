use super::handlers::*;
use crate::shared::app_state::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn device_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_devices))
        .route("/", post(create_device))
        .route("/{id}", get(get_device_by_id))
        .route("/{id}", put(update_device))
        .route("/{id}", delete(delete_device))
        .route("/batch/{user_id}", put(update_many_devices))
}
