//! This module defines the authentication service trait used to abstract
//! user login and registration logic.

use crate::{
    auth::dto::AuthUserDto,
    common::{
        error::AppError,
        jwt::{AuthBody, AuthPayload},
    },
};

#[async_trait::async_trait]
/// Trait defining the contract for authentication-related operations.
/// Implementors are responsible for handling user creation and login logic.
pub trait AuthServiceTrait: Send + Sync {
    /// Registers a new user authentication entry.
    async fn create_user_auth(&self, auth_user: AuthUserDto) -> Result<(), AppError>;

    /// Authenticates a user and returns a JWT token payload on success.
    async fn login_user(&self, auth_payload: AuthPayload) -> Result<AuthBody, AppError>;
}
