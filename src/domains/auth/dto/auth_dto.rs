use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct AuthUserDto {
    pub user_id: String,
    pub password: String,
}
