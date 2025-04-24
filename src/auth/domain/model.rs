//! This module defines the `UserAuth` model used for representing
//! authentication data tied to a user.

use serde::{Deserialize, Serialize};

/// Represents a user's authentication information, including hashed password.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuth {
    pub user_id: String,
    pub password_hash: String,
}
