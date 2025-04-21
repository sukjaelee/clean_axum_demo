use super::dto::{CreateUserMultipartDto, UpdateUserDto};
use super::{model::User, repository::UserRepository};
use async_trait::async_trait;
use sqlx::{MySql, Pool, Transaction};

pub struct UserRepo;

#[async_trait]
impl UserRepository for UserRepo {
    async fn find_all(&self, pool: Pool<MySql>) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT
                u.id,
                u.username,
                u.email,
                u.created_by,
                u.created_at,
                u.modified_by,
                u.modified_at,
                uf.id as file_id,
                uf.origin_file_name
            FROM users u
            LEFT JOIN uploaded_files uf 
                   ON uf.user_id = u.id and uf.file_type = 'profile_picture'
            "#
        )
        .fetch_all(&pool)
        .await?;

        Ok(users)
    }

    async fn find_by_id(&self, pool: Pool<MySql>, id: String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                u.id,
                u.username,
                u.email,
                u.created_by,
                u.created_at,
                u.modified_by,
                u.modified_at,
                uf.id as file_id,
                uf.origin_file_name
            FROM users u
            LEFT JOIN uploaded_files uf 
                   ON uf.user_id = u.id and uf.file_type = 'profile_picture'
           WHERE u.id = ?"#,
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(user)
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, MySql>,
        user: CreateUserMultipartDto,
    ) -> Result<String, sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO users (username, email, created_by, modified_by)
             VALUES (?, ?, ?, ?)"#,
            user.username,
            user.email,
            user.modified_by,
            user.modified_by,
        )
        .execute(&mut **tx)
        .await?;

        let record = sqlx::query!(
            r#"SELECT id FROM users WHERE username = ? AND email = ?"#,
            user.username,
            user.email
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(record.id)
    }

    async fn update(
        &self,
        tx: &mut Transaction<'_, MySql>,
        id: String,
        user: UpdateUserDto,
    ) -> Result<Option<User>, sqlx::Error> {
        let existing = sqlx::query_as!(
            User,
            r#"
            SELECT
                u.id,
                u.username,
                u.email,
                u.created_by,
                u.created_at,
                u.modified_by,
                u.modified_at,
                uf.id as file_id,
                uf.origin_file_name
            FROM users u
            LEFT JOIN uploaded_files uf 
                   ON uf.user_id = u.id and uf.file_type = 'profile_picture'
           WHERE u.id = ?"#,
            id
        )
        .fetch_optional(&mut **tx)
        .await?;

        if existing.is_some() {
            sqlx::query!(
                r#"UPDATE users SET username = ?, email = ?, modified_by = ?, modified_at = NOW() WHERE id = ?"#,
                user.username,
                user.email,
                user.modified_by,
                id
            )
            .execute(&mut **tx)
            .await?;

            let updated_user = sqlx::query_as!(
                User,
                r#"
                SELECT
                    u.id,
                    u.username,
                    u.email,
                    u.created_by,
                    u.created_at,
                    u.modified_by,
                    u.modified_at,
                    uf.id as file_id,
                    uf.origin_file_name
                FROM users u
                LEFT JOIN uploaded_files uf 
                       ON uf.user_id = u.id and uf.file_type = 'profile_picture'
               WHERE u.id = ?"#,
                id
            )
            .fetch_one(&mut **tx)
            .await?;

            return Ok(Some(updated_user));
        }

        Ok(None)
    }

    async fn delete(
        &self,
        tx: &mut Transaction<'_, MySql>,
        id: String,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(r#"DELETE FROM users WHERE id = ?"#, id)
            .execute(&mut **tx)
            .await?;

        Ok(res.rows_affected() > 0)
    }
}
