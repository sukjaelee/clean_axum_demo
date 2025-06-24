//! This module defines the authentication service trait used to abstract
//! user login and registration logic.

use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    common::{
        error::AppError,
        jwt::{AuthBody, AuthPayload},
    },
    domains::auth::dto::auth_dto::AuthUserDto,
};

#[async_trait::async_trait]
/// Trait defining the contract for authentication-related operations.
/// Implementors are responsible for handling user creation and login logic.
pub trait AuthServiceTrait: Send + Sync {
    /// constructor for the service.
    fn create_service(pool: PgPool) -> Arc<dyn AuthServiceTrait>
    where
        Self: Sized;

    /// Registers a new user authentication entry.
    async fn create_user_auth(&self, auth_user: AuthUserDto) -> Result<(), AppError>;

    /// Authenticates a user and returns a JWT token payload on success.
    async fn login_user(&self, auth_payload: AuthPayload) -> Result<AuthBody, AppError>;
}
