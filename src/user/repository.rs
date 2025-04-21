use super::{
    dto::{CreateUserMultipartDto, UpdateUserDto},
    model::User,
};

use async_trait::async_trait;
use sqlx::{MySql, Pool, Transaction};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self, pool: Pool<MySql>) -> Result<Vec<User>, sqlx::Error>;
    async fn find_by_id(&self, pool: Pool<MySql>, id: String) -> Result<Option<User>, sqlx::Error>;
    async fn create(
        &self,
        tx: &mut Transaction<'_, MySql>,
        user: CreateUserMultipartDto,
    ) -> Result<String, sqlx::Error>;
    async fn update(
        &self,
        tx: &mut Transaction<'_, MySql>,
        id: String,
        user: UpdateUserDto,
    ) -> Result<Option<User>, sqlx::Error>;
    async fn delete(
        &self,
        tx: &mut Transaction<'_, MySql>,
        id: String,
    ) -> Result<bool, sqlx::Error>;
}
