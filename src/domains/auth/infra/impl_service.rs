use std::sync::Arc;

use crate::{
    common::{
        error::AppError,
        hash_util,
        jwt::{make_jwt_token, AuthBody, AuthPayload},
    },
    domains::auth::{
        domain::{model::UserAuth, repository::UserAuthRepository, service::AuthServiceTrait},
        dto::auth_dto::AuthUserDto,
        infra::impl_repository::UserAuthRepo,
    },
};

use sqlx::PgPool;

/// Service for handling user authentication
/// and authorization logic.
#[derive(Clone)]
pub struct AuthService {
    pool: PgPool,
    repo: Arc<dyn UserAuthRepository + Send + Sync>,
}

/// Implementation of the AuthService
#[async_trait::async_trait]
impl AuthServiceTrait for AuthService {
    /// constructor for the service.
    fn create_service(pool: PgPool) -> Arc<dyn AuthServiceTrait> {
        Arc::new(Self {
            pool,
            repo: Arc::new(UserAuthRepo {}),
        })
    }

    /// It hashes the password and stores it in the database.
    async fn create_user_auth(&self, auth_user: AuthUserDto) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        let password_hash =
            hash_util::hash_password(&auth_user.password).map_err(|_| AppError::InternalError)?;

        let user_auth = UserAuth {
            user_id: auth_user.user_id,
            password_hash,
        };

        match self.repo.create(&mut tx, user_auth).await {
            Ok(()) => {
                tx.commit().await?;
                Ok(())
            }
            Err(err) => {
                tracing::error!("Error creating user auth: {err}");
                tx.rollback().await?;
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// Authenticates a user by checking the provided credentials
    /// against the stored credentials in the database.
    /// If the credentials are valid, it generates a JWT token for the user.
    /// If the credentials are invalid, it returns an error.
    async fn login_user(&self, auth_payload: AuthPayload) -> Result<AuthBody, AppError> {
        if auth_payload.client_id.is_empty() || auth_payload.client_secret.is_empty() {
            return Err(AppError::MissingCredentials);
        }

        let user_auth = self
            .repo
            .find_by_user_name(self.pool.clone(), auth_payload.client_id.clone())
            .await
            .map_err(AppError::DatabaseError)?;

        let user_auth = user_auth.ok_or(AppError::UserNotFound)?;

        if !hash_util::verify_password(&user_auth.password_hash, &auth_payload.client_secret) {
            return Err(AppError::WrongCredentials);
        }

        let token = make_jwt_token(&user_auth.user_id).map_err(|_| AppError::InternalError)?;

        Ok(AuthBody::new(token))
    }
}
