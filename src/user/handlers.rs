use std::collections::HashMap;

use crate::{
    common::{app_state::AppState, dto::RestApiResponse, error::AppError, jwt::Claims},
    file::dto::UpdateFile,
    user::dto::{CreateUserMultipartDto, UpdateUserDto, UserDto},
};

use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
    Extension, Json,
};

use validator::Validate;

#[utoipa::path(
    get,
    path = "/user/{id}",
    responses((status = 200, description = "Get user by ID", body = UserDto)),
    tag = "Users"
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.get_user_by_id(id).await?;
    Ok(RestApiResponse::success(user))
}

#[utoipa::path(
    get,
    path = "/user",
    responses((status = 200, description = "List all users", body = [UserDto])),
    tag = "Users"
)]
pub async fn get_users(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let users = state.user_service.get_users().await?;
    Ok(RestApiResponse::success(users))
}

#[utoipa::path(
    post,
    path = "/user",
    request_body(
        content = CreateUserMultipartDto,
        content_type = "multipart/form-data",
        description = "User creation with optional profile picture upload"
    ),
    responses((status = 200, description = "Create a new user", body = UserDto)),
    tag = "Users"
)]
pub async fn create_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let modified_by = claims.sub.clone().to_string();

    // Variables to hold multipart fields.
    let mut fields = HashMap::new();
    let mut profile_picture_filename: Option<String> = None;
    let mut profile_picture_content_type: Option<String> = None;
    let mut profile_picture_data: Option<Vec<u8>> = None;

    // Helper closure to map multipart errors to AppError.
    let map_err_internal = |err| {
        tracing::error!("Multipart error: {}", err);
        AppError::InternalError
    };

    while let Some(field) = multipart.next_field().await.map_err(map_err_internal)? {
        match field.name() {
            Some("profile_picture") => {
                profile_picture_filename = field.file_name().map(|s| s.to_string());
                profile_picture_content_type = field.content_type().map(|mime| mime.to_string());
                profile_picture_data =
                    Some(field.bytes().await.map_err(map_err_internal)?.to_vec());
            }
            Some(name) => {
                let name = name.to_string();
                let value = field.text().await.map_err(map_err_internal)?;
                fields.insert(name, value);
            }
            None => {}
        }
    }

    // Validate required fields.
    let username = fields
        .remove("username")
        .ok_or(AppError::ValidationError("Missing username".into()))?;
    let email = fields
        .remove("email")
        .ok_or(AppError::ValidationError("Missing email".into()))?;

    // Prepare the CreateUser DTO.
    let create_user = CreateUserMultipartDto {
        username,
        email,
        modified_by: modified_by.clone(),
        profile_picture: None,
    };

    // Validate the CreateUser DTO.
    create_user
        .validate()
        .map_err(|err| AppError::ValidationError(format!("Invalid input: {}", err)))?;

    let mut upload_file = None;

    // If a profile picture was uploaded, handle it.
    if let (Some(data), Some(filename), Some(content_type)) = (
        profile_picture_data,
        profile_picture_filename,
        profile_picture_content_type,
    ) {
        upload_file = Some(UpdateFile {
            user_id: None,
            original_filename: filename,
            data,
            content_type: content_type.clone(),
            modified_by: modified_by.clone(),
        });
    }

    let user = state
        .user_service
        .create_user(create_user, upload_file.as_mut())
        .await?;

    Ok(RestApiResponse::success(user))
}

#[utoipa::path(
    put,
    path = "/user/{id}",
    request_body = UpdateUserDto,
    responses((status = 200, description = "Update user", body = UserDto)),
    tag = "Users"
)]
pub async fn update_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<UpdateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|err| {
        tracing::error!("Validation error: {err}");
        AppError::ValidationError(format!("Invalid input: {}", err))
    })?;

    // Set the modified_by field to the current user's ID.
    let mut payload = payload;
    payload.modified_by = claims.sub.clone().to_string();

    let user = state.user_service.update_user(id, payload).await?;
    Ok(RestApiResponse::success(user))
}

#[utoipa::path(
    delete,
    path = "/user/{id}",
    responses((status = 200, description = "User deleted")),
    tag = "Users"
)]
pub async fn delete_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let message = state.user_service.delete_user(id).await?;
    Ok(RestApiResponse::success_with_message(message, ()))
}
