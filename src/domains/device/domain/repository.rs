// This module defines the `DeviceRepository` trait, which abstracts
// the database operations related to device management.

use crate::domains::device::dto::device_dto::{
    CreateDeviceDto, UpdateDeviceDto, UpdateManyDevicesDto,
};

use super::model::Device;

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

#[async_trait]
/// Trait representing repository-level operations for device entities.
/// Provides an interface for data persistence and retrieval of device records.
pub trait DeviceRepository: Send + Sync {
    /// Retrieves all devices from the database.
    async fn find_all(&self, pool: PgPool) -> Result<Vec<Device>, sqlx::Error>;

    /// Finds a device by its unique identifier.
    async fn find_by_id(&self, pool: PgPool, id: String) -> Result<Option<Device>, sqlx::Error>;

    /// Creates a new device record in the database within the given transaction.
    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        device: CreateDeviceDto,
    ) -> Result<Device, sqlx::Error>;

    /// Updates an existing device record with new data.
    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
        device: UpdateDeviceDto,
    ) -> Result<Option<Device>, sqlx::Error>;

    /// Updates multiple devices for a given user with the specified changes.
    async fn update_many(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: String,
        modified_by: String,
        update_devices: UpdateManyDevicesDto,
    ) -> Result<(), sqlx::Error>;

    /// Deletes a device record by its ID.
    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
    ) -> Result<bool, sqlx::Error>;
}
