use super::handlers::*;
use crate::shared::app_state::AppState;
use axum::{routing::post, Router};

pub fn user_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_user))
        .route("/register", post(create_user_auth))
}
