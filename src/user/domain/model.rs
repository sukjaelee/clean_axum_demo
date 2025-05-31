// Domain model definition for user entities.
// Represents core user attributes and metadata in the system.

use sqlx::FromRow;
use time::OffsetDateTime;

/// Domain model representing a user in the application.
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub modified_by: Option<String>,
    pub modified_at: Option<OffsetDateTime>,
    pub file_id: Option<String>,
    pub origin_file_name: Option<String>,
}
