use crate::domains::file::{
    domain::{model::UploadedFile, repository::FileRepository},
    dto::file_dto::CreateFile,
};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

pub struct FileRepo;

const FIND_FILE_INFO_QUERY: &str = r#"
    SELECT id, user_id, file_name, origin_file_name, file_relative_path, file_url,
            content_type, file_size, file_type, created_by, 
            created_at, 
            modified_by,
            modified_at
    FROM uploaded_files 
    WHERE id = $1
"#;

#[async_trait]
impl FileRepository for FileRepo {
    async fn create_file(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        file: CreateFile,
    ) -> Result<UploadedFile, sqlx::Error> {
        let id = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            INSERT INTO uploaded_files
              (id, user_id, file_name, origin_file_name, file_relative_path, file_url, content_type, file_size, file_type, created_by, modified_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            id.clone(),
            file.user_id.clone(),
            file.file_name.clone(),
            file.origin_file_name,
            file.file_relative_path,
            file.file_url,
            file.content_type,
            file.file_size as i64,
            file.file_type.to_string(),
            file.modified_by.clone(),
            file.modified_by
        )
        .execute(&mut **tx)
        .await?;

        let inserted_file = sqlx::query_as::<_, UploadedFile>(FIND_FILE_INFO_QUERY)
            .bind(id)
            .fetch_one(&mut **tx)
            .await?;

        Ok(inserted_file)
    }

    async fn find_by_user_id(
        &self,
        pool: PgPool,
        user_id: String,
    ) -> Result<Option<UploadedFile>, sqlx::Error> {
        let uploaded_file = sqlx::query_as!(
            UploadedFile,
            r#"
            SELECT id, user_id, file_name, origin_file_name, file_relative_path, file_url,
                content_type, file_size, file_type, created_by,
                created_at,
                modified_by,
                modified_at
            FROM uploaded_files 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(uploaded_file)
    }

    async fn find_by_id(
        &self,
        pool: PgPool,
        id: String,
    ) -> Result<Option<UploadedFile>, sqlx::Error> {
        let uploaded_file = sqlx::query_as::<_, UploadedFile>(FIND_FILE_INFO_QUERY)
            .bind(id)
            .fetch_optional(&pool)
            .await?;

        Ok(uploaded_file)
    }

    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(r#"DELETE FROM uploaded_files WHERE id = $1"#, id)
            .execute(&mut **tx)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
