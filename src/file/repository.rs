use super::{dto::CreateFile, model::UploadedFile};
use async_trait::async_trait;
use sqlx::{MySql, Pool, Transaction};

#[async_trait]
pub trait FileRepository {
    async fn create_file(
        &self,
        tx: &mut Transaction<'_, MySql>,
        file: CreateFile,
    ) -> Result<UploadedFile, sqlx::Error>;

    async fn find_by_id(
        &self,
        pool: Pool<MySql>,
        id: String,
    ) -> Result<Option<UploadedFile>, sqlx::Error>;

    async fn find_by_user_id(
        &self,
        pool: Pool<MySql>,
        user_id: String,
    ) -> Result<Option<UploadedFile>, sqlx::Error>;

    async fn delete(
        &self,
        tx: &mut Transaction<'_, MySql>,
        id: String,
    ) -> Result<bool, sqlx::Error>;
}
