//! This module defines the `FileServiceTrait` used for managing
//! file upload, retrieval, and deletion operations.

use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    common::{config::Config, error::AppError},
    domains::file::dto::file_dto::{UploadFileDto, UploadedFileDto},
};

#[async_trait]
/// Trait defining the contract for file-related operations.
/// Used to abstract file handling logic such as uploading,
/// retrieving metadata, and deleting files.
pub trait FileServiceTrait: Send + Sync {
    /// constructor for the service.
    fn create_service(config: Config, pool: PgPool) -> Arc<dyn FileServiceTrait>
    where
        Self: Sized;

    /// Processes a profile picture upload within an active transaction.
    /// Returns the uploaded file's metadata on success.
    async fn process_profile_picture_upload(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        upload_file_dto: &UploadFileDto,
    ) -> Result<Option<UploadedFileDto>, AppError>;

    /// Retrieves file metadata by its file ID.
    async fn get_file_metadata(&self, file_id: String)
        -> Result<Option<UploadedFileDto>, AppError>;

    /// Deletes a file by its file ID and returns a confirmation message.
    async fn delete_file(&self, file_id: String) -> Result<String, AppError>;
}
