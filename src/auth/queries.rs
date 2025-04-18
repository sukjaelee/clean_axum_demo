use super::{model::UserAuth, repository::UserAuthRepository};
use async_trait::async_trait;
use sqlx::{MySql, Pool, Transaction};

pub struct UserAuthRepo;

#[async_trait]
impl UserAuthRepository for UserAuthRepo {
    async fn find_by_user_name(
        &self,
        pool: Pool<MySql>,
        user_name: String,
    ) -> Result<Option<UserAuth>, sqlx::Error> {
        let result = sqlx::query_as!(
            UserAuth,
            r#"SELECT ua.user_id, ua.password_hash 
                 FROM user_auth ua
                 JOIN users u ON ua.user_id = u.id
                 WHERE u.username = ?"#,
            user_name
        )
        .fetch_optional(&pool)
        .await?;

        Ok(result)
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, MySql>,
        user_auth: UserAuth,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO user_auth (user_id, password_hash)
             VALUES (?, ?)"#,
            user_auth.user_id,
            user_auth.password_hash,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
