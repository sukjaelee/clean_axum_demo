use crate::file::model::file_model::FileType;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFile {
    pub user_id: String,
    pub file_name: String,
    pub origin_file_name: String,
    pub file_relative_path: String,
    pub file_url: String,
    pub content_type: String,
    pub file_size: u32,
    pub file_type: FileType,
    pub modified_by: String,
}
