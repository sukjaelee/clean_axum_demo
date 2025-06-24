use crate::common::app_state::AppState;
use axum::{routing::post, Router};

use super::handlers;

use utoipa::OpenApi;

/// Import the necessary modules for OpenAPI documentation generation
#[derive(OpenApi)]
#[openapi(
    paths(
        super::handlers::login_user,
        super::handlers::create_user_auth,
    ),
    components(schemas(
        crate::domains::auth::dto::auth_dto::AuthUserDto,
        crate::common::jwt::AuthPayload,
        crate::common::jwt::AuthBody,
    )),
    tags(
        (name = "UserAuth", description = "User authentication endpoints")
    )
)]
/// This struct is used to generate OpenAPI documentation for the user authentication routes.
pub struct UserAuthApiDoc;

/// This function creates a router for the user authentication routes.
/// It defines the routes and their corresponding handlers.
pub fn user_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(handlers::login_user))
        .route("/register", post(handlers::create_user_auth))
}
