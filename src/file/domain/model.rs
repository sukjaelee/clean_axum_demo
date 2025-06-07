//! Domain model definitions related to uploaded files.
//! This includes the `FileType` enum and `UploadedFile` struct,
//! used to represent file metadata in the business logic layer.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    decode::Decode,
    postgres::{PgTypeInfo, PgValueRef},
    FromRow, Postgres, Type,
};
use std::{fmt, str::FromStr};
use utoipa::ToSchema;

use crate::common::error::AppError;

/// Enum representing different categories of files stored in the system.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    ProfilePicture,
    Document,
    Video,
    Other,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::ProfilePicture => write!(f, "profile_picture"),
            FileType::Document => write!(f, "document"),
            FileType::Video => write!(f, "video"),
            FileType::Other => write!(f, "other"),
        }
    }
}

impl FromStr for FileType {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "profile_picture" => Ok(FileType::ProfilePicture),
            "document" => Ok(FileType::Document),
            "video" => Ok(FileType::Video),
            "other" => Ok(FileType::Other),
            _ => Err(AppError::ValidationError(format!("Invalid file type: {s}"))),
        }
    }
}

impl From<String> for FileType {
    fn from(s: String) -> Self {
        s.parse()
            .unwrap_or_else(|_| panic!("Invalid file type: {}", s))
    }
}

impl<'r> Decode<'r, Postgres> for FileType {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(FileType::from_str(s)?)
    }
}

impl Type<Postgres> for FileType {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

/// Domain model representing metadata for a file uploaded by a user.
#[derive(Debug, Clone, FromRow)]
pub struct UploadedFile {
    pub id: String,
    pub user_id: String,
    pub file_name: String,
    pub origin_file_name: String,
    pub file_relative_path: String,
    pub file_url: String,
    pub content_type: String,
    pub file_size: i64,
    pub file_type: FileType,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_by: Option<String>,
    pub modified_at: DateTime<Utc>,
}
