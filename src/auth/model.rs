use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuth {
    pub user_id: String,
    pub password_hash: String,
}
