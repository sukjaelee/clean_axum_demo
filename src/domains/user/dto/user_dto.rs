use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::domains::user::domain::model::User;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub created_by: Option<String>,
    #[serde(with = "crate::common::ts_format::option")]
    pub created_at: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
    #[serde(with = "crate::common::ts_format::option")]
    pub modified_at: Option<DateTime<Utc>>,
    pub file_id: Option<String>,
    pub origin_file_name: Option<String>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_by: user.created_by,
            created_at: user.created_at,
            modified_by: user.modified_by,
            modified_at: user.modified_at,
            file_id: user.file_id,
            origin_file_name: user.origin_file_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchUserDto {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserMultipartDto {
    #[validate(length(max = 64, message = "Username cannot exceed 64 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub modified_by: String,
    // Optional profile picture file provided as binary data.
    #[allow(dead_code)]
    #[schema(value_type = String, format = "binary", example = "profile_picture.png")]
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUserDto {
    #[validate(length(max = 64, message = "Username cannot exceed 64 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub modified_by: String,
}
