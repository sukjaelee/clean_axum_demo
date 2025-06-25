//! This module defines the `FileRepository` trait, which provides
//! an abstraction over database operations for managing uploaded files.

use crate::domains::file::dto::file_dto::CreateFileDto;

use super::model::UploadedFile;

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

#[async_trait]
/// Trait representing repository-level operations for uploaded file metadata.
/// Enables persistence, retrieval, and deletion of file records through database interactions.
pub trait FileRepository {
    /// Inserts a new file record into the database using a transaction.
    async fn create_file(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        file: CreateFileDto,
    ) -> Result<UploadedFile, sqlx::Error>;

    /// Finds a file record by its unique identifier.
    async fn find_by_id(
        &self,
        pool: PgPool,
        id: String,
    ) -> Result<Option<UploadedFile>, sqlx::Error>;

    /// Finds a file record associated with a specific user ID.
    async fn find_by_user_id(
        &self,
        pool: PgPool,
        user_id: String,
    ) -> Result<Option<UploadedFile>, sqlx::Error>;

    /// Deletes a file record by its unique identifier using a transaction.
    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
    ) -> Result<bool, sqlx::Error>;
}
