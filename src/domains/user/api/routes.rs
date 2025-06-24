use super::handlers::*;
use crate::{
    common::app_state::AppState,
    domains::user::dto::user_dto::{CreateUserMultipartDto, SearchUserDto, UpdateUserDto, UserDto},
};

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_user_by_id,
        get_users,
        get_user_list,
        create_user,
        update_user,
        delete_user,
    ),
    components(schemas(UserDto, SearchUserDto, CreateUserMultipartDto, UpdateUserDto)),
    tags(
        (name = "Users", description = "User management endpoints")
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&UserApiDoc)
)]
/// This struct is used to generate OpenAPI documentation for the user routes.
pub struct UserApiDoc;

impl utoipa::Modify for UserApiDoc {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("Input your `<your‑jwt>`"))
                    .build(),
            ),
        )
    }
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user))
        .route("/list", post(get_user_list))
        .route("/{id}", get(get_user_by_id))
        .route("/{id}", put(update_user))
        .route("/{id}", delete(delete_user))
}
