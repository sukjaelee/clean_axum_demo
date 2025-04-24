use super::{
    domain::{model::FileType, repository::FileRepository, service::FileServiceTrait},
    dto::{CreateFile, UpdateFile, UploadedFileDto},
    queries::FileRepo,
};
use crate::common::{config::Config, error::AppError};

use regex::Regex;

use sqlx::MySqlPool;
use sqlx::{MySql, Transaction};
use std::path::Path as FilePath;
use std::sync::Arc;

use async_trait::async_trait;

/// Service struct for handling file-related operations
/// such as uploading, deleting, and fetching files.
/// It uses a repository pattern to abstract the data access layer.
#[derive(Clone)]
pub struct FileService {
    config: Config,
    pool: MySqlPool,
    repo: Arc<dyn FileRepository + Send + Sync>,
}

/// Implementation of the constructor for the FileService.
/// Provides a factory function that returns a boxed trait object.
impl FileService {
    /// Creates an `Arc`-wrapped `FileService` implementing `FileServiceTrait`.
    /// This allows the service to be used behind a trait object for dependency injection.
    pub fn create_service(config: Config, pool: MySqlPool) -> Arc<dyn FileServiceTrait> {
        Arc::new(Self {
            config,
            pool,
            repo: Arc::new(FileRepo {}),
        })
    }
}

/// Implementation of the FileService struct
#[async_trait]
impl FileServiceTrait for FileService {
    /// Uploads a profile picture for a user.
    /// Validates the file, writes it to disk, and stores its metadata in the database.
    /// Returns the uploaded file's metadata.
    async fn process_profile_picture_upload(
        &self,
        tx: &mut Transaction<'_, MySql>,
        upload_file: &UpdateFile,
    ) -> Result<Option<UploadedFileDto>, AppError> {
        FileService::validate_file_upload(
            &self.config,
            &upload_file.data,
            &upload_file.original_filename,
        )?;

        let (unique_filename, file_relative_path, file_path) =
            self.build_file_path(&upload_file.original_filename);

        self.write_file_to_disk(&file_path, &upload_file.data)?;

        let file_url = format!(
            "{}/profile/{}",
            self.config.assets_private_url, &unique_filename
        );

        let create_file_dto = CreateFile {
            user_id: upload_file.user_id.clone(),
            file_name: unique_filename,
            origin_file_name: upload_file.original_filename.clone(),
            file_relative_path,
            file_url,
            content_type: upload_file.content_type.clone(),
            file_size: upload_file.data.len() as u32,
            file_type: FileType::ProfilePicture,
            modified_by: upload_file.modified_by.clone(),
        };

        self.repo
            .create_file(tx, create_file_dto)
            .await
            .map_err(|err| {
                tracing::error!("Error uploading file: {}", err);
                AppError::DatabaseError(err)
            })?;

        if let Some(user_id) = &upload_file.user_id {
            self.get_file_by_user(user_id.clone()).await
        } else {
            Err(AppError::ValidationError("User ID is missing".into()))
        }
    }

    /// Retrieves the metadata of a file by its id.
    async fn get_file_metadata(
        &self,
        file_id: String,
    ) -> Result<Option<UploadedFileDto>, AppError> {
        let uploaded_file = self
            .repo
            .find_by_id(self.pool.clone(), file_id.clone())
            .await
            .map_err(|err| {
                tracing::error!("Error retrieving file: {}", err);
                AppError::DatabaseError(err)
            });

        match uploaded_file {
            Ok(Some(file)) => Ok(Some(UploadedFileDto::from(file))),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Deletes a file by its id.
    /// Removes the file from the filesystem and deletes its metadata from the database.
    /// Returns a success message if the deletion was successful.
    async fn delete_file(&self, file_id: String) -> Result<String, AppError> {
        let mut tx = self.pool.begin().await?;

        let to_delete_file = self
            .repo
            .find_by_id(self.pool.clone(), file_id.clone())
            .await
            .map_err(|err| {
                tracing::error!("Error retrieving file: {}", err);
                AppError::DatabaseError(err)
            })?;

        if to_delete_file.is_none() {
            return Err(AppError::NotFound("File not found".into()));
        }

        let deletion_result = self.repo.delete(&mut tx, file_id).await.map_err(|err| {
            tracing::error!("Error deleting file: {}", err);
            AppError::DatabaseError(err)
        })?;

        if !deletion_result {
            return Err(AppError::NotFound("File not found".into()));
        }

        let file_path = FilePath::new(self.config.assets_private_path.as_str())
            .join(to_delete_file.unwrap().file_relative_path);

        if std::fs::remove_file(&file_path).is_err() {
            tracing::error!(
                "Error deleting file from filesystem: {}",
                file_path.to_str().unwrap()
            );
            return Err(AppError::InternalError);
        }

        tx.commit().await?;

        Ok("File deleted successfully".into())
    }
}

/// Internal helper methods defined on `FileService`.
impl FileService {
    /// Retrieves file metadata associated with a given user ID from the repository.
    async fn get_file_by_user(&self, user_id: String) -> Result<Option<UploadedFileDto>, AppError> {
        let uploaded_file = self
            .repo
            .find_by_user_id(self.pool.clone(), user_id)
            .await
            .map_err(|err| {
                tracing::error!("Error retrieving file: {}", err);
                AppError::DatabaseError(err)
            });

        match uploaded_file {
            Ok(Some(file)) => Ok(Some(UploadedFileDto::from(file))),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Ensures the generated filename is unique within the given directory.
    fn generate_unique_filename(original: &str, base_dir: &str) -> String {
        let path = FilePath::new(original);
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let mut candidate = if ext.is_empty() {
            format!("{}", stem)
        } else {
            format!("{}.{}", stem, ext)
        };

        let mut count = 1;
        let base = FilePath::new(base_dir);

        while base.join(&candidate).exists() {
            candidate = if ext.is_empty() {
                format!("{}({})", stem, count)
            } else {
                format!("{}({}).{}", stem, count, ext)
            };
            count += 1;
        }

        candidate
    }

    /// Performs validation checks on file size, name, and extension.
    pub fn validate_file_upload(
        config: &Config,
        data: &[u8],
        original_filename: &str,
    ) -> Result<(), AppError> {
        if data.len() > config.asset_max_size {
            tracing::error!(
                "File size exceeds the maximum limit of {}.",
                config.asset_max_size
            );
            return Err(AppError::FileSizeExceeded);
        }

        if original_filename.contains("..") || original_filename.contains("/") {
            tracing::error!("Invalid file name: {}", original_filename);
            return Err(AppError::InvalidFileName);
        }

        let pattern = format!(r"(?i)^.*\.({})$", config.asset_allowed_extensions);
        let re = Regex::new(&pattern).map_err(|err| {
            tracing::error!("Error compiling regex: {}", err);
            AppError::InternalError
        })?;

        if !re.is_match(original_filename) {
            tracing::error!("Unsupported file extension: {}", original_filename);
            return Err(AppError::UnsupportedFileExtension);
        }

        Ok(())
    }

    /// Constructs a unique filename, relative path, and absolute disk path for the upload.
    fn build_file_path(&self, original_filename: &str) -> (String, String, std::path::PathBuf) {
        let base_dir = self.config.assets_private_path.as_str();
        let base_dir_with_profile =
            FilePath::new(base_dir).join(FileType::ProfilePicture.to_string());

        let unique_filename = FileService::generate_unique_filename(
            original_filename,
            base_dir_with_profile.to_str().unwrap(),
        );
        let file_path = base_dir_with_profile.join(&unique_filename);

        let relative_path = format!("{}/{}", FileType::ProfilePicture, unique_filename);
        (unique_filename, relative_path, file_path)
    }

    /// Writes the file's byte data to the disk, creating directories as needed.
    fn write_file_to_disk(&self, file_path: &FilePath, data: &[u8]) -> Result<(), AppError> {
        let parent = file_path.parent().ok_or(AppError::InternalError)?;
        std::fs::create_dir_all(parent).map_err(|err| {
            tracing::error!("Error creating directory: {}", err);
            AppError::InternalError
        })?;

        std::fs::write(file_path, data).map_err(|err| {
            tracing::error!("Error writing file: {}", err);
            AppError::InternalError
        })?;

        if !file_path.exists() {
            tracing::error!("File was not written successfully.");
            return Err(AppError::InternalError);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::config::Config, file::service::FileService};

    #[tokio::test]
    async fn test_validate_file_upload() {
        let config = Config {
            asset_max_size: 5 * 1024 * 1024, // 5MB
            asset_allowed_extensions: "jpg|jpeg|png|gif".to_string(),
            ..Default::default()
        };

        let valid_file = vec![0; 4 * 1024 * 1024]; // 4MB
        let invalid_file = vec![0; 6 * 1024 * 1024]; // 6MB

        assert!(FileService::validate_file_upload(&config, &valid_file, "test.jpg.sh").is_err());
        assert!(FileService::validate_file_upload(&config, &valid_file, "test.jpg").is_ok());
        assert!(FileService::validate_file_upload(&config, &invalid_file, "test.jpg").is_err());
    }
}
