use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use time::OffsetDateTime;
use utoipa::ToSchema;

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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "profile_picture" => Ok(FileType::ProfilePicture),
            "document" => Ok(FileType::Document),
            "video" => Ok(FileType::Video),
            "other" => Ok(FileType::Other),
            _ => Err(()),
        }
    }
}

impl From<String> for FileType {
    fn from(s: String) -> Self {
        s.parse()
            .unwrap_or_else(|_| panic!("Invalid file type: {}", s))
    }
}

#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub id: String,
    pub user_id: String,
    pub file_name: String,
    pub origin_file_name: String,
    pub file_relative_path: String,
    pub file_url: String,
    pub content_type: String,
    pub file_size: u32,
    pub file_type: FileType,
    pub created_by: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub modified_by: Option<String>,
    pub modified_at: Option<OffsetDateTime>,
}
