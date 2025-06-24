use crate::domains::user::{
    domain::{model::User, repository::UserRepository},
    dto::user_dto::{CreateUserMultipartDto, SearchUserDto, UpdateUserDto},
};
use async_trait::async_trait;

use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

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
    WHERE 1=1
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
    WHERE u.id = $1
    "#;

#[async_trait]
impl UserRepository for UserRepo {
    async fn find_all(&self, pool: PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(FIND_USER_QUERY)
            .fetch_all(&pool)
            .await?;
        Ok(users)
    }

    async fn find_list(
        &self,
        pool: PgPool,
        search_user_dto: SearchUserDto,
    ) -> Result<Vec<User>, sqlx::Error> {
        let mut builder = QueryBuilder::<_>::new(FIND_USER_QUERY);

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

        let query = builder.build_query_as::<User>();
        let users = query.fetch_all(&pool).await?;
        Ok(users)
    }

    async fn find_by_id(&self, pool: PgPool, id: String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(FIND_USER_INFO_QUERY)
            .bind(id)
            .fetch_optional(&pool)
            .await?;
        Ok(user)
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: CreateUserMultipartDto,
    ) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
                INSERT INTO users (id, username, email, created_by, modified_by)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            id.clone(),
            user.username.clone(),
            user.email.clone(),
            user.modified_by.clone(),
            user.modified_by
        )
        .execute(&mut **tx)
        .await?;

        Ok(id)
    }

    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
        user: UpdateUserDto,
    ) -> Result<Option<User>, sqlx::Error> {
        let existing = sqlx::query_as::<_, User>(FIND_USER_INFO_QUERY)
            .bind(id.clone())
            .fetch_optional(&mut **tx)
            .await?;

        if existing.is_some() {
            sqlx::query!(
                r#"
                UPDATE users 
                SET username = $1,
                    email = $2, 
                    modified_by = $3, 
                    modified_at = NOW() 
                WHERE id = $4
                "#,
                user.username.clone(),
                user.email.clone(),
                user.modified_by.clone(),
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
        tx: &mut Transaction<'_, Postgres>,
        id: String,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, id)
            .execute(&mut **tx)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
