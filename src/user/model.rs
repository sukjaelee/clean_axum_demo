use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub created_by: Option<String>,
    #[serde(with = "crate::shared::ts_format::option")]
    pub created_at: Option<OffsetDateTime>,
    pub modified_by: Option<String>,
    #[serde(with = "crate::shared::ts_format::option")]
    pub modified_at: Option<OffsetDateTime>,
    pub file_id: Option<String>,
    pub origin_file_name: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: String::default(),
            username: String::default(),
            email: None,
            created_by: None,
            created_at: None,
            modified_by: None,
            modified_at: None,
            file_id: None,
            origin_file_name: None,
        }
    }
}
