use super::{dto::CreateFile, model::UploadedFile, repository::FileRepository};
use async_trait::async_trait;
use sqlx::{mysql::MySql, Pool, Transaction};

pub struct FileRepo;

#[async_trait]
impl FileRepository for FileRepo {
    async fn create_file(
        &self,
        tx: &mut Transaction<'_, MySql>,
        file: CreateFile,
    ) -> Result<UploadedFile, sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO uploaded_files 
            (user_id, file_name, origin_file_name, file_relative_path, file_url, content_type, file_size, file_type, created_by, modified_by) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            file.user_id,
            file.file_name,
            file.origin_file_name,
            file.file_relative_path, 
            file.file_url,
            file.content_type,
            file.file_size as i32,
            file.file_type.to_string(),
            file.modified_by,  // using modified_by as created_by here
            file.modified_by
        )
        .execute(&mut **tx)
        .await?;

        let inserted_file = sqlx::query_as!(UploadedFile,
            r#"SELECT id, user_id, file_name, origin_file_name, file_relative_path, file_url, content_type, file_size, file_type, created_by, created_at, modified_by, updated_at as modified_at 
            FROM uploaded_files 
            WHERE user_id = ? and file_name = ?"#,
            file.user_id,
            file.file_name
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(inserted_file)
    }

    async fn find_by_user_id(
        &self,
        pool: Pool<MySql>,
        user_id: String,
    ) -> Result<Option<UploadedFile>, sqlx::Error> {
        let uploaded_file = sqlx::query_as!(UploadedFile,
            r#"SELECT id, user_id, file_name, origin_file_name, file_relative_path, file_url, content_type, file_size, file_type, created_by, created_at, modified_by, updated_at as modified_at 
            FROM uploaded_files 
            WHERE user_id = ?"#,
            user_id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(uploaded_file)
    }

    async fn find_by_id(
        &self,
        pool: Pool<MySql>,
        id: String,
    ) -> Result<Option<UploadedFile>, sqlx::Error> {
        let uploaded_file = sqlx::query_as!(UploadedFile,
            r#"SELECT id, user_id, file_name, origin_file_name, file_relative_path, file_url, content_type, file_size, file_type, created_by, created_at, modified_by, updated_at as modified_at 
            FROM uploaded_files 
            WHERE id = ?"#,
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(uploaded_file)
    }

    async fn delete(&self, tx: &mut Transaction<'_, MySql>, id: String) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(r#"DELETE FROM uploaded_files WHERE id = ?"#, id)
            .execute(&mut **tx)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
