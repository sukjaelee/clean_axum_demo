use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::auth::domain::model::UserAuth;
use crate::auth::domain::repository::UserAuthRepository;
pub struct UserAuthRepo;

#[async_trait]
impl UserAuthRepository for UserAuthRepo {
    async fn find_by_user_name(
        &self,
        pool: PgPool,
        user_name: String,
    ) -> Result<Option<UserAuth>, sqlx::Error> {
        let result = sqlx::query_as!(
            UserAuth,
            r#"
            SELECT ua.user_id, ua.password_hash
              FROM user_auth ua
              JOIN users u ON ua.user_id = u.id
              WHERE u.username = $1
            "#,
            user_name
        )
        .fetch_optional(&pool)
        .await?;

        Ok(result)
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_auth: UserAuth,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_auth 
            (user_id, password_hash)
            VALUES 
            ($1, $2)
            "#,
            user_auth.user_id,
            user_auth.password_hash
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
