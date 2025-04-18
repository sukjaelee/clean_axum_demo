use super::model::UserAuth;

use async_trait::async_trait;
use sqlx::{MySql, Pool, Transaction};

#[async_trait]
pub trait UserAuthRepository: Send + Sync {
    async fn find_by_user_name(
        &self,
        pool: Pool<MySql>,
        user_name: String,
    ) -> Result<Option<UserAuth>, sqlx::Error>;

    async fn create(
        &self,
        tx: &mut Transaction<'_, MySql>,
        user_auth: UserAuth,
    ) -> Result<(), sqlx::Error>;
}
