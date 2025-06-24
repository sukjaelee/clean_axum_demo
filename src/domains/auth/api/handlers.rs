use crate::{
    common::{
        app_state::AppState,
        dto::RestApiResponse,
        error::AppError,
        jwt::{AuthBody, AuthPayload},
    },
    domains::auth::dto::auth_dto::AuthUserDto,
};
use axum::extract::State;
use axum::{response::IntoResponse, Json};

/// this function creates a router for creating user authentication registration
/// it will create a new user in the database
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = AuthUserDto,
    responses((status = 200, description = "Create user authentication", body = AuthUserDto)),
    tag = "UserAuth"
)]
pub async fn create_user_auth(
    State(state): State<AppState>,
    Json(payload): Json<AuthUserDto>,
) -> Result<impl IntoResponse, AppError> {
    state.auth_service.create_user_auth(payload).await?;
    Ok(RestApiResponse::success(()))
}

/// this function creates a router for login user
/// it will return a JWT token if the user is authenticated
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = AuthPayload,
    responses((status = 200, description = "Login user", body = AuthBody)),
    tag = "UserAuth"
)]
pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let auth_body = state.auth_service.login_user(payload).await?;
    Ok(RestApiResponse::success(auth_body))
}
