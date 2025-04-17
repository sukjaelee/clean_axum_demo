use crate::auth::model::user_auth_model::UserAuth;
use crate::shared::app_state::AppState;
use crate::shared::error::AppError;
use crate::shared::hash_util;
use crate::shared::jwt::{make_jwt_token, AuthBody, AuthPayload};
use axum::extract::State;
use axum::{response::IntoResponse, Json};
use serde_json::json;

use utoipa::OpenApi;

use super::user_auth_dto::AuthUser;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user_auth, login_user
    ),
    components(schemas(AuthUser)),
    tags(
        (name = "UserAuth", description = "User authentication endpoints")
    )
)]
pub struct UserAuthApiDoc;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = AuthUser,
    responses((status = 200, description = "Create user authentication", body = AuthUser)),
    tag = "UserAuth"
)]
pub async fn create_user_auth(
    State(state): State<AppState>,
    Json(payload): Json<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.pool.begin().await?;

    let password_hash =
        hash_util::hash_password(&payload.password).map_err(|_| AppError::InternalError)?;

    let user_auth = UserAuth {
        user_id: payload.user_id,
        password_hash: password_hash,
    };

    match state.user_auth_repo.create(&mut tx, user_auth).await {
        Ok(()) => {
            tx.commit().await?; // Commit the transaction
            Ok(Json(json!({ "result": "success" })).into_response())
        }
        Err(err) => {
            tracing::error!("Error creating user auth: {err}");
            tx.rollback().await?; // Rollback the transaction
            Err(AppError::DatabaseError(err))
        }
    }
}

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
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    // Check if the user exists in the database
    let user_auth = state
        .user_auth_repo
        .find_by_user_name(state.pool, payload.client_id.clone())
        .await
        .map_err(|err| AppError::DatabaseError(err))?;

    if user_auth.is_none() {
        return Err(AppError::UserNotFound);
    }

    // Check if the password is correct
    let user_auth = user_auth.unwrap();
    if !hash_util::verify_password(&user_auth.password_hash, &payload.client_secret) {
        return Err(AppError::WrongCredentials);
    }

    // Create the authorization token
    let token = make_jwt_token(&user_auth.user_id).unwrap();

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
