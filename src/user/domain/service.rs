//! This module defines the `UserServiceTrait` responsible for user-related business logic.
//! It abstracts operations such as user creation, retrieval, update, and deletion.

use crate::{
    common::error::AppError,
    file::dto::UpdateFile,
    user::dto::{CreateUserMultipartDto, UpdateUserDto, UserDto},
};

use async_trait::async_trait;

#[async_trait]
/// Trait defining business operations for user management.
/// Provides methods for interacting with users in a domain-agnostic way.
pub trait UserServiceTrait: Send + Sync {
    /// Retrieves a user by their unique identifier.
    async fn get_user_by_id(&self, id: String) -> Result<UserDto, AppError>;

    /// Retrieves all users.
    async fn get_users(&self) -> Result<Vec<UserDto>, AppError>;

    /// Creates a new user with optional profile picture upload.
    async fn create_user(
        &self,
        create_user: CreateUserMultipartDto,
        upload_file: Option<&mut UpdateFile>,
    ) -> Result<UserDto, AppError>;

    /// Updates an existing user with the given payload.
    async fn update_user(&self, id: String, payload: UpdateUserDto) -> Result<UserDto, AppError>;

    /// Deletes a user by their unique identifier.
    async fn delete_user(&self, id: String) -> Result<String, AppError>;
}
