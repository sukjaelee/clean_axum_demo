//! This module defines the `UserRepository` trait, which abstracts
//! the database operations related to user entities.

use crate::domains::user::dto::user_dto::{CreateUserMultipartDto, SearchUserDto, UpdateUserDto};

use super::model::User;

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

#[async_trait]
/// Trait representing repository-level operations for user entities.
/// Provides methods for creating, retrieving, updating, and deleting users in the database.
pub trait UserRepository: Send + Sync {
    /// Retrieves all users from the database.
    async fn find_all(&self, pool: PgPool) -> Result<Vec<User>, sqlx::Error>;

    /// Finds a user by their unique identifier.
    async fn find_by_id(&self, pool: PgPool, id: String) -> Result<Option<User>, sqlx::Error>;

    /// Finds user list by condition
    async fn find_list(
        &self,
        pool: PgPool,
        search_user_dto: SearchUserDto,
    ) -> Result<Vec<User>, sqlx::Error>;

    /// Creates a new user record using the provided data within an active transaction.
    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: CreateUserMultipartDto,
    ) -> Result<String, sqlx::Error>;

    /// Updates an existing user record using the provided data.
    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
        user: UpdateUserDto,
    ) -> Result<Option<User>, sqlx::Error>;

    /// Deletes a user by their unique identifier within an active transaction.
    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
    ) -> Result<bool, sqlx::Error>;
}
