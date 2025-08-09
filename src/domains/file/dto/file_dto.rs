use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use simple_dto_mapper_derive::DtoFrom;
use utoipa::ToSchema;

use crate::domains::file::domain::model::{FileType, UploadedFile};

#[derive(PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct FileDto {
    pub content_type: String,
    pub original_filename: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFileDto {
    pub user_id: Option<String>,
    pub file_name: String,
    pub origin_file_name: String,
    pub file_relative_path: String,
    pub file_url: String,
    pub content_type: String,
    pub file_size: u32,
    pub file_type: FileType,
    pub modified_by: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadFileDto {
    pub file: FileDto,
    pub user_id: Option<String>,
    pub modified_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, DtoFrom)]
#[dto(from = UploadedFile)]
pub struct UploadedFileDto {
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
    #[serde(with = "crate::common::ts_format")]
    pub created_at: DateTime<Utc>,
    pub modified_by: Option<String>,
    #[serde(with = "crate::common::ts_format")]
    pub modified_at: DateTime<Utc>,
}
