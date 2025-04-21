use time::OffsetDateTime;

#[derive(Debug, Clone)]
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
