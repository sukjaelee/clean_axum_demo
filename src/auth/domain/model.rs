//! This module defines the `UserAuth` model used for representing
//! authentication data tied to a user.

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Represents a user's authentication information, including hashed password.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserAuth {
    pub user_id: String,
    pub password_hash: String,
}
