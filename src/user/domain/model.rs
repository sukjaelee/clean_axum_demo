use chrono::NaiveDateTime;
use sqlx::FromRow;
/// Domain model representing a user in the application.
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub modified_by: Option<String>,
    pub modified_at: Option<NaiveDateTime>,
    pub file_id: Option<String>,
    pub origin_file_name: Option<String>,
}
