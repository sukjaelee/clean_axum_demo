use std::collections::HashMap;

use crate::{app::FORBIDDEN_PATTERNS, common::error::AppError, domains::file::FileDto};

const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";

/// Internal helper to parse multipart form data into maps of text values and file fields.
///
/// Consumes the multipart stream, validating field names and file names against forbidden patterns,
/// and restricting file uploads to allowed extensions. Retains all occurrences of each text field.
/// Returns a tuple:
/// - `fields`: `HashMap<String, Vec<String>>` mapping each field name to its list of text values.
/// - `files`: `HashMap<String, Vec<FileDto>>` mapping each field name to its uploaded files.
async fn parse_multipart_internal(
    mut multipart: axum::extract::Multipart,
    asset_allowed_extensions_pattern: &regex::Regex,
) -> Result<
    (
        std::collections::HashMap<String, Vec<String>>,
        std::collections::HashMap<String, Vec<FileDto>>,
    ),
    AppError,
> {
    let mut fields: HashMap<String, Vec<String>> = HashMap::new();
    let mut files: HashMap<String, Vec<FileDto>> = HashMap::new();

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

        if let Some(filename) = field.file_name() {
            let original_filename = filename.to_string();
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
            files.entry(name).or_default().push(FileDto {
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
            fields.entry(name).or_default().push(text);
        }
    }

    Ok((fields, files))
}

/// Parses multipart form data into a map of single string values and a map of file fields.
///
/// Consumes the multipart stream, validating field names and file names against forbidden patterns,
/// and restricting file uploads to allowed extensions. For text fields, if a field appears multiple times,
/// the last value is retained. Returns:
/// - `fields`: `HashMap<String, String>` mapping each field name to its text value.
/// - `files`: `HashMap<String, Vec<FileDto>>` mapping each field name to its uploaded files.
pub async fn parse_multipart_to_maps(
    multipart: axum::extract::Multipart,
    asset_allowed_extensions_pattern: &regex::Regex,
) -> Result<
    (
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, Vec<FileDto>>,
    ),
    AppError,
> {
    let (multi_fields, files) =
        parse_multipart_internal(multipart, asset_allowed_extensions_pattern).await?;
    let fields = multi_fields
        .into_iter()
        .map(|(k, mut v)| {
            let last = v.pop().unwrap_or_default();
            (k, last)
        })
        .collect();
    Ok((fields, files))
}

/// Parses multipart form data into maps of text values and file fields.
///
/// Consumes the multipart stream, validating field names and file names against forbidden patterns,
/// and restricting file uploads to allowed extensions. All occurrences of a text field are retained
/// in the returned map.
/// Returns:
/// - `fields`: `HashMap<String, Vec<String>>` mapping each field name to its list of text values.
/// - `files`: `HashMap<String, Vec<FileDto>>` mapping each field name to its uploaded files.
pub async fn parse_multipart_to_multi_maps(
    multipart: axum::extract::Multipart,
    asset_allowed_extensions_pattern: &regex::Regex,
) -> Result<
    (
        std::collections::HashMap<String, Vec<String>>,
        std::collections::HashMap<String, Vec<FileDto>>,
    ),
    AppError,
> {
    parse_multipart_internal(multipart, asset_allowed_extensions_pattern).await
}
