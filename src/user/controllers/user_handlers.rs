use crate::file::controllers::file_handler::process_profile_picture_upload;
use crate::shared::app_state::AppState;
use crate::shared::jwt::Claims;
use crate::user::controllers::user_dto::UpdateUser;
use crate::user::model::user_model::User;
use crate::{shared::error::AppError, user::controllers::user_dto::CreateUserMultipart};
use axum::extract::{Multipart, State};
use axum::Extension;
use axum::{response::IntoResponse, Json};
use serde_json::json;

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;
use validator::Validate;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_user_by_id,
        get_users,
        create_user,
        update_user,
        delete_user,
    ),
    components(schemas(User, CreateUserMultipart, UpdateUser)),
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

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses((status = 200, description = "Get user by ID", body = User)),
    tag = "Users"
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match state.user_repo.find_by_id(state.pool, id).await {
        Ok(Some(user)) => Ok(Json(json!({ "user": user })).into_response()),
        Ok(None) => Ok((AppError::NotFound("User not found".into())).into_response()),
        Err(err) => {
            tracing::error!("Error retrieving user: {err}");
            Err(AppError::DatabaseError(err))
        }
    }
}

#[utoipa::path(
    get,
    path = "/users",
    responses((status = 200, description = "List all users", body = [User])),
    tag = "Users"
)]
pub async fn get_users(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("----Hello, {}!", claims.sub);

    match state.user_repo.find_all(state.pool).await {
        Ok(users) => Ok(Json(json!({ "users": users })).into_response()),
        Err(err) => {
            tracing::error!("Error fetching users: {err}");
            Err(AppError::DatabaseError(err))
        }
    }
}

#[utoipa::path(
    post,
    path = "/users",
    request_body(
        content = CreateUserMultipart,
        content_type = "multipart/form-data",
        description = "User creation with optional profile picture upload"
    ),
    responses((status = 200, description = "Create a new user", body = User)),
    tag = "Users"
)]
pub async fn create_user(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    // Variables to hold multipart fields.
    let mut username: Option<String> = None;
    let mut email: Option<String> = None;
    let mut modified_by: Option<String> = None;
    let mut profile_picture_data: Option<Vec<u8>> = None;
    let mut profile_picture_filename: Option<String> = None;
    let mut profile_picture_content_type: Option<String> = None;

    // Helper closure to map multipart errors to AppError.
    let map_err_internal = |err| {
        tracing::error!("Multipart error: {}", err);
        AppError::InternalError
    };

    // Process each field in the multipart form.
    while let Some(field) = multipart.next_field().await.map_err(map_err_internal)? {
        match field.name() {
            Some("username") => {
                username = Some(field.text().await.map_err(map_err_internal)?);
            }
            Some("email") => {
                email = Some(field.text().await.map_err(map_err_internal)?);
            }
            Some("modified_by") => {
                modified_by = Some(field.text().await.map_err(map_err_internal)?);
            }
            Some("profile_picture") => {
                // Capture metadata before consuming the field
                profile_picture_filename = field.file_name().map(|s| s.to_string());
                profile_picture_content_type = field.content_type().map(|mime| mime.to_string());

                // Now consume the field to get the file bytes
                profile_picture_data =
                    Some(field.bytes().await.map_err(map_err_internal)?.to_vec());
            }
            _ => {}
        }
    }

    // Validate required fields.
    let username = username.ok_or(AppError::ValidationError("Missing username".into()))?;
    let email = email.ok_or(AppError::ValidationError("Missing email".into()))?;
    let modified_by = modified_by.ok_or(AppError::ValidationError("Missing modified_by".into()))?;

    let mut tx = state.pool.begin().await?;

    // Prepare the CreateUser DTO.
    let create_user_dto = CreateUserMultipart {
        username,
        email,
        modified_by: modified_by.clone(),
        profile_picture: None,
    };

    // Validate the CreateUser DTO.
    create_user_dto
        .validate()
        .map_err(|err| AppError::ValidationError(format!("Invalid input: {}", err)))?;

    // Create the user record.
    let user_id = match state.user_repo.create(&mut tx, create_user_dto).await {
        Ok(user_id) => user_id,
        Err(err) => {
            tracing::error!("Error creating user: {err}");
            tx.rollback().await?; // Rollback the transaction
            return Err(AppError::DatabaseError(err));
        }
    };

    // If a profile picture was uploaded, handle it.
    if let (Some(data), Some(filename), Some(content_type)) = (
        profile_picture_data,
        profile_picture_filename,
        profile_picture_content_type,
    ) {
        process_profile_picture_upload(
            data,
            filename,
            content_type,
            user_id.clone(),
            modified_by.clone(),
            &state,
            &mut tx,
        )
        .await?;
    }

    tx.commit().await?;

    match state.user_repo.find_by_id(state.pool, user_id).await {
        Ok(Some(user)) => Ok(Json(json!({ "user": user })).into_response()),
        Ok(None) => Ok((AppError::NotFound("User not found".into())).into_response()),
        Err(err) => {
            tracing::error!("Error retrieving user: {err}");
            Err(AppError::DatabaseError(err))
        }
    }
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    request_body = UpdateUser,
    responses((status = 200, description = "Update user", body = User)),
    tag = "Users"
)]
pub async fn update_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<UpdateUser>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.pool.begin().await?;

    payload.validate().map_err(|err| {
        tracing::error!("Validation error: {err}");
        AppError::ValidationError(format!("Invalid input: {}", err))
    })?;

    match state
        .user_repo
        .update(&mut tx, id.to_string(), payload)
        .await
    {
        Ok(Some(user)) => {
            tx.commit().await?; // Commit the transaction
            Ok(Json(json!({ "user": user })).into_response())
        }
        Ok(None) => {
            tx.rollback().await?; // Rollback the transaction
            Ok((AppError::NotFound("User not found".into())).into_response())
        }
        Err(err) => {
            tracing::error!("Error updating user: {err}");
            tx.rollback().await?; // Rollback the transaction
            Err(AppError::DatabaseError(err))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses((status = 200, description = "User deleted")),
    tag = "Users"
)]
pub async fn delete_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.pool.begin().await?;

    match state.user_repo.delete(&mut tx, id.to_string()).await {
        Ok(true) => {
            tx.commit().await?; // Commit the transaction
            Ok((axum::http::StatusCode::OK, "User deleted").into_response())
        }
        Ok(false) => {
            tx.rollback().await?; // Rollback the transaction
            Ok((AppError::NotFound("User not found".into())).into_response())
        }
        Err(err) => {
            tracing::error!("Error deleting user: {err}");
            tx.rollback().await?; // Rollback the transaction
            Err(AppError::DatabaseError(err))
        }
    }
}
