use crate::file::controllers::file_dto::CreateFile;
use crate::file::model::file_model::{FileType, UploadedFile};
use crate::shared::app_state::AppState;
use crate::shared::config::Config;
use crate::shared::error::AppError;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use regex::Regex;
use serde_json::json;
use sqlx::{MySql, Transaction};
use std::path::Path as FilePath;
use tokio_util::io::ReaderStream;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        serve_protected_file,
        delete_file,
    ),
    components(schemas(UploadedFile)),
    tags(
        (name = "Files", description = "File management endpoints")
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&FileApiDoc)
)]

/// FileApiDoc is used to generate OpenAPI documentation for the file API.
pub struct FileApiDoc;

impl utoipa::Modify for FileApiDoc {
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

fn generate_unique_filename(original: &str, base_dir: &str) -> String {
    let path = FilePath::new(original);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Start with the original filename.
    let mut candidate = if ext.is_empty() {
        format!("{}", stem)
    } else {
        format!("{}.{}", stem, ext)
    };

    let mut count = 1;
    // Build the candidate file path within the base directory.
    let base = FilePath::new(base_dir);

    while base.join(&candidate).exists() {
        candidate = if ext.is_empty() {
            format!("{}({})", stem, count)
        } else {
            format!("{}({}).{}", stem, count, ext)
        };
        count += 1;
    }

    candidate
}

pub fn validate_file_upload(
    config: &Config,
    data: &[u8],
    original_filename: &str,
) -> Result<(), AppError> {
    // Validate the file size (limit to 5MB in this example).

    if data.len() > config.asset_max_size {
        tracing::error!("File size exceeds the maximum limit of 5MB.");
        return Err(AppError::FileSizeExceeded);
    }

    // Validate the file name by checking for invalid characters.
    if original_filename.contains("..") || original_filename.contains("/") {
        tracing::error!("Invalid file name: {}", original_filename);
        return Err(AppError::InvalidFileName);
    }

    // Validate the file extension using a regex pattern.
    // The regex pattern is case-insensitive and matches the allowed extensions.
    // The allowed extensions are defined in the config file.
    let pattern = format!(r"(?i)^.*\.({})$", config.asset_allowed_extensions);
    let re = Regex::new(&pattern).map_err(|err| {
        tracing::error!("Error compiling regex: {}", err);
        AppError::InternalError
    })?;

    if !re.is_match(original_filename) {
        tracing::error!("Unsupported file extension: {}", original_filename);
        return Err(AppError::UnsupportedFileExtension);
    }

    Ok(())
}

// Process the upload of a profile picture.
pub async fn process_profile_picture_upload(
    data: Vec<u8>,
    original_filename: String,
    content_type: String,
    user_id: String,
    modified_by: String,
    state: &AppState,
    tx: &mut Transaction<'_, MySql>,
) -> Result<Option<UploadedFile>, AppError> {
    // Validate the file upload.
    validate_file_upload(&state.config, &data, &original_filename)?;

    let assets_private_path = state.config.assets_private_path.clone();
    let base_dir = assets_private_path.as_str();

    let base_dir_with_profile = FilePath::new(base_dir).join(FileType::ProfilePicture.to_string());

    // Ensure the base directory exists.
    // Create the directory if it doesn't exist.

    std::fs::create_dir_all(&base_dir_with_profile).map_err(|err| {
        tracing::error!("Error creating directory: {}", err);
        AppError::InternalError
    })?;

    let unique_filename = generate_unique_filename(
        &original_filename,
        base_dir_with_profile.clone().to_str().unwrap(),
    );
    let file_path = base_dir_with_profile.join(unique_filename.clone());

    std::fs::write(&file_path, &data).map_err(|err| {
        tracing::error!("Error writing file: {}", err);
        AppError::InternalError
    })?;

    // Ensure the file was written successfully.
    if !file_path.exists() {
        tracing::error!("File was not written successfully.");
        return Err(AppError::InternalError);
    }

    // Build the file relative path.
    // This is the path relative to the base directory.
    let file_relative_path = format!(
        "{}/{}",
        FileType::ProfilePicture.to_string(),
        unique_filename
    );

    // Build the file URL.
    let assets_private_url = state.config.assets_private_url.clone();
    let file_url = format!("{}/profile/{}", assets_private_url, unique_filename);

    // Create the file DTO.
    let create_file_dto = CreateFile {
        user_id: user_id.clone(),
        file_name: unique_filename.clone(),
        origin_file_name: original_filename.clone(),
        file_relative_path: file_relative_path.clone(),
        file_url: file_url.clone(),
        content_type: content_type.clone(),
        file_size: data.len() as u32,
        file_type: FileType::ProfilePicture,
        modified_by,
    };

    state
        .file_repo
        .create_file(tx, create_file_dto)
        .await
        .map_err(|err| {
            tracing::error!("Error uploading file: {}", err);
            AppError::DatabaseError(err)
        })?;

    // return the uploaded file information
    let file_option: Option<UploadedFile> =
        get_file_by_user(State(state.clone()), user_id.clone()).await?;
    Ok(file_option)
}

async fn get_file_by_user(
    State(state): State<AppState>,
    user_id: String,
) -> Result<Option<UploadedFile>, AppError> {
    let pool = state.pool.clone();
    state
        .file_repo
        .find_by_user_id(pool, user_id)
        .await
        .map_err(|err| {
            tracing::error!("Error retrieving file: {}", err);
            AppError::DatabaseError(err)
        })
}

#[utoipa::path(
    get,
    path = "/files/{file_id}",
    responses((status = 200, description = "Serve protected file")),
    tag = "Files"
)]
/// Serve a protected file from the server's filesystem.
pub async fn serve_protected_file(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let pool = state.pool.clone();

    let file_metadata = state
        .file_repo
        .find_by_id(pool, file_id.clone())
        .await
        .map_err(|err| {
            tracing::error!("Error retrieving file: {}", err);
            AppError::DatabaseError(err)
        })?;

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
        .body(axum::body::Body::from(body))
        .map_err(|err| {
            tracing::error!("Error building response: {}", err);
            AppError::InternalError
        })?;

    Ok(response)
}

#[utoipa::path(
    delete,
    path = "/files/{file_id}",
    responses((status = 200, description = "Delete file")),
    tag = "Files"
)]
/// Delete a file from the server's filesystem and database.
pub async fn delete_file(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.pool.begin().await?;

    // Check if the file exists before attempting to delete it.
    let to_delete_file = state
        .file_repo
        .find_by_id(state.pool.clone(), file_id.clone())
        .await
        .map_err(|err| {
            tracing::error!("Error retrieving file: {}", err);
            AppError::DatabaseError(err)
        })?;

    // If the file doesn't exist, return a 404 error.
    if to_delete_file.is_none() {
        return Ok(Json(json!({
            "message": "File not found"
        })));
    }
    // Delete the file from the database.
    let deletion_result = state
        .file_repo
        .delete(&mut tx, file_id)
        .await
        .map_err(|err| {
            tracing::error!("Error deleting file: {}", err);
            AppError::DatabaseError(err)
        })?;

    // Ensure the file is deleted from the filesystem as well.
    let assets_private_path = state.config.assets_private_path.clone();
    let base_dir = assets_private_path.as_str();
    // Build the full file system path.
    let file_path = FilePath::new(base_dir).join(to_delete_file.unwrap().file_relative_path);

    if std::fs::remove_file(&file_path).is_err() {
        tracing::error!(
            "Error deleting file from filesystem: {}",
            file_path.to_str().unwrap()
        );
        return Err(AppError::InternalError);
    }

    tx.commit().await?;

    if deletion_result {
        Ok(Json(json!({
            "message": "File deleted successfully"
        })))
    } else {
        Ok(Json(json!({
            "message": "File not found"
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::config::Config;

    #[tokio::test]
    async fn test_validate_file_upload() {
        let config = Config {
            asset_max_size: 5 * 1024 * 1024, // 5MB
            asset_allowed_extensions: "jpg|jpeg|png|gif".to_string(),
            ..Default::default()
        };

        let valid_file = vec![0; 4 * 1024 * 1024]; // 4MB
        let invalid_file = vec![0; 6 * 1024 * 1024]; // 6MB

        assert!(validate_file_upload(&config, &valid_file, "test.jpg.sh").is_err());
        assert!(validate_file_upload(&config, &valid_file, "test.jpg").is_ok());
        assert!(validate_file_upload(&config, &invalid_file, "test.jpg").is_err());
    }
}
