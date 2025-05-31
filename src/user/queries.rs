use super::dto::SearchUserDto;
use crate::user::{
    domain::{model::User, repository::UserRepository},
    dto::{CreateUserMultipartDto, UpdateUserDto},
};
use async_trait::async_trait;
use sqlx::FromRow;
use sqlx::{mysql::MySqlRow, MySql, Pool, QueryBuilder, Transaction};

pub struct UserRepo;

const FIND_USER_QUERY: &str = r#"
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
    "#;

const FIND_USER_INFO_QUERY: &str = r#"
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
    WHERE u.id = ?
    "#;

#[async_trait]
impl UserRepository for UserRepo {
    async fn find_all(&self, pool: Pool<MySql>) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(FIND_USER_QUERY)
            .fetch_all(&pool)
            .await?;

        Ok(users)
    }

    async fn find_list(
        &self,
        pool: Pool<MySql>,
        search_user_dto: SearchUserDto,
    ) -> Result<Vec<User>, sqlx::Error> {
        let mut builder = QueryBuilder::<MySql>::new(FIND_USER_QUERY);

        if let Some(s) = search_user_dto
            .id
            .as_deref()
            .filter(|s| !s.trim().is_empty())
        {
            builder.push(" AND u.id = ");
            builder.push_bind(s);
        }

        if let Some(s) = search_user_dto
            .username
            .as_deref()
            .filter(|s| !s.trim().is_empty())
        {
            builder.push(" AND u.username like ");
            builder.push_bind(format!("%{}%", s));
        }

        let rows: Vec<MySqlRow> = builder.build().fetch_all(&pool).await?;

        let users = rows
            .into_iter()
            .map(|row: MySqlRow| User::from_row(&row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(users)
    }

    async fn find_by_id(&self, pool: Pool<MySql>, id: String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(FIND_USER_INFO_QUERY)
            .bind(id)
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
        let existing = sqlx::query_as::<_, User>(FIND_USER_INFO_QUERY)
            .bind(id.clone())
            .fetch_optional(&mut **tx)
            .await?;

        if existing.is_some() {
            sqlx::query!(
                r#"UPDATE users SET username = ?, email = ?, modified_by = ?, modified_at = NOW() WHERE id = ?"#,
                user.username,
                user.email,
                user.modified_by,
                id.clone()
            )
            .execute(&mut **tx)
            .await?;

            let updated_user = sqlx::query_as::<_, User>(FIND_USER_INFO_QUERY)
                .bind(id)
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
