use crate::common::{app_state::AppState, dto::RestApiResponse, error::AppError};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

use std::path::Path as FilePath;
use tokio_util::io::ReaderStream;

/// This function serves a protected file from the server's filesystem.
/// It will return the file as a response with the appropriate content type and headers.
/// If the file is not found, it will return a 404 error.
#[utoipa::path(
    get,
    path = "/file/{file_id}",
    responses((status = 200, description = "Serve protected file")),
    tag = "Files"
)]
/// Serve a protected file from the server's filesystem.
pub async fn serve_protected_file(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let file_metadata = state.file_service.get_file_metadata(file_id).await?;

    // If the file is not found, return a 404.
    let file_metadata = file_metadata.ok_or_else(|| AppError::NotFound("File not found".into()))?;

    // Build the full file system path.
    let assets_private_path = state.config.assets_private_path.clone();
    let base_dir = assets_private_path.as_str();

    let file_path = FilePath::new(base_dir).join(file_metadata.file_relative_path);

    if !file_path.exists() {
        return Err(AppError::NotFound("File not found".into()));
    }

    // Open and stream the file.
    let file = tokio::fs::File::open(file_path).await.map_err(|err| {
        tracing::error!("Error opening file: {}", err);
        AppError::InternalError
    })?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    // Build a full response with content type header set to the file's MIME type.
    // Here we use file_metadata.content_type that should contain a valid MIME string.
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, file_metadata.content_type)
        .header(
            header::CONTENT_DISPOSITION,
            format!(
                "attachment; filename=\"{}\"",
                file_metadata.origin_file_name
            ),
        )
        .body(body)
        .map_err(|err| {
            tracing::error!("Error building response: {}", err);
            AppError::InternalError
        })?;

    Ok(response)
}

/// This function deletes a file from the server's filesystem and database.
/// It will return a success message if the deletion is successful, or an error if not.
#[utoipa::path(
    delete,
    path = "/file/{file_id}",
    responses((status = 200, description = "Delete file")),
    tag = "Files"
)]
/// Delete a file from the server's filesystem and database.
pub async fn delete_file(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let message = state.file_service.delete_file(file_id).await?;
    Ok(RestApiResponse::success_with_message(message, ()))
}
