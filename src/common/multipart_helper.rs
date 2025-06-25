use std::collections::HashMap;

use crate::{app::FORBIDDEN_PATTERNS, common::error::AppError, domains::file::FileDto};

const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";

pub async fn parse_multipart_to_maps(
    mut multipart: axum::extract::Multipart,
    asset_allowed_extensions_pattern: &regex::Regex,
) -> Result<
    (
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, Vec<FileDto>>,
    ),
    AppError,
> {
    let mut fields: HashMap<String, String> = std::collections::HashMap::new();
    let mut files: HashMap<String, Vec<FileDto>> = std::collections::HashMap::new();

    // Helper closure to map multipart errors to AppError.
    let map_err_internal = |err| {
        tracing::error!("Multipart error: {}", err);
        AppError::InternalError
    };

    while let Some(field) = multipart.next_field().await.map_err(map_err_internal)? {
        let name = field
            .name()
            .ok_or_else(|| AppError::ValidationError("Field name is missing".to_string()))?
            .to_string();

        if FORBIDDEN_PATTERNS.iter().any(|re| re.is_match(&name)) {
            tracing::error!("Invalid field name: {}", name);
            return Err(AppError::Forbidden);
        }

        if field.file_name().is_some() {
            let original_filename = field.file_name().unwrap_or("unnamed").to_string();

            if original_filename.contains("..") || original_filename.contains("/") {
                tracing::error!("Invalid file name: {}", original_filename);
                return Err(AppError::InvalidFileName);
            }

            if FORBIDDEN_PATTERNS
                .iter()
                .any(|re| re.is_match(&original_filename))
            {
                tracing::error!("Invalid file name: {}", original_filename);
                return Err(AppError::Forbidden);
            }

            if !asset_allowed_extensions_pattern.is_match(&original_filename) {
                tracing::error!("Unsupported file extension: {}", original_filename);
                return Err(AppError::UnsupportedFileExtension);
            }

            let content_type = field
                .content_type()
                .unwrap_or(APPLICATION_OCTET_STREAM)
                .to_string();
            let data = field.bytes().await.map_err(map_err_internal)?.to_vec();

            files.entry(name).or_insert_with(Vec::new).push(FileDto {
                content_type,
                original_filename,
                data,
            });
        } else {
            let text = field.text().await.map_err(map_err_internal)?;

            if FORBIDDEN_PATTERNS.iter().any(|re| re.is_match(&text)) {
                tracing::error!("Invalid text field value: {}", text);
                return Err(AppError::Forbidden);
            }

            fields.insert(name, text);
        }
    }
    Ok((fields, files))
}
