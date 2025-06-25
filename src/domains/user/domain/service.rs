//! This module defines the `UserServiceTrait` responsible for user-related business logic.
//! It abstracts operations such as user creation, retrieval, update, and deletion.

use crate::{
    common::error::AppError,
    domains::file::dto::file_dto::UploadFileDto,
    domains::user::dto::user_dto::{CreateUserMultipartDto, SearchUserDto, UpdateUserDto, UserDto},
};

use crate::domains::file::FileServiceTrait;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
/// Trait defining business operations for user management.
/// Provides methods for interacting with users in a domain-agnostic way.
pub trait UserServiceTrait: Send + Sync {
    /// constructor for the service.
    fn create_service(
        pool: PgPool,
        file_service: Arc<dyn FileServiceTrait>,
    ) -> Arc<dyn UserServiceTrait>
    where
        Self: Sized;

    /// Retrieves a user by their unique identifier.
    async fn get_user_by_id(&self, id: String) -> Result<UserDto, AppError>;

    /// Retrieves user list by condition
    async fn get_user_list(&self, search_user_dto: SearchUserDto)
        -> Result<Vec<UserDto>, AppError>;

    /// Retrieves all users.
    async fn get_users(&self) -> Result<Vec<UserDto>, AppError>;

    /// Creates a new user with optional profile picture upload.
    async fn create_user(
        &self,
        create_user: CreateUserMultipartDto,
        upload_file_dto: Option<&mut UploadFileDto>,
    ) -> Result<UserDto, AppError>;

    /// Updates an existing user with the given payload.
    async fn update_user(&self, id: String, payload: UpdateUserDto) -> Result<UserDto, AppError>;

    /// Deletes a user by their unique identifier.
    async fn delete_user(&self, id: String) -> Result<String, AppError>;
}
