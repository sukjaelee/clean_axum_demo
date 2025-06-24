//! This module defines the `DeviceServiceTrait` which encapsulates the business logic
//! for managing devices in the system.

use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    common::error::AppError,
    domains::device::dto::device_dto::{
        CreateDeviceDto, DeviceDto, UpdateDeviceDto, UpdateManyDevicesDto,
    },
};

#[async_trait::async_trait]
/// Trait defining the contract for device-related business operations.
/// This includes creating, retrieving, updating, and deleting devices,
/// as well as batch updates for user-associated devices.
pub trait DeviceServiceTrait: Send + Sync {
    /// constructor for the service.
    fn create_service(pool: PgPool) -> Arc<dyn DeviceServiceTrait>
    where
        Self: Sized;

    /// Retrieves a device by its unique ID.
    async fn get_device_by_id(&self, id: String) -> Result<DeviceDto, AppError>;

    /// Retrieves a list of all devices.
    async fn get_devices(&self) -> Result<Vec<DeviceDto>, AppError>;

    /// Creates a new device from the provided payload.
    async fn create_device(&self, payload: CreateDeviceDto) -> Result<DeviceDto, AppError>;

    /// Updates an existing device with new data.
    async fn update_device(
        &self,
        id: String,
        payload: UpdateDeviceDto,
    ) -> Result<DeviceDto, AppError>;

    /// Deletes a device by its ID.
    async fn delete_device(&self, id: String) -> Result<String, AppError>;

    /// Applies updates to multiple devices owned by a user.
    async fn update_many_devices(
        &self,
        user_id: String,
        modified_by: String,
        payload: UpdateManyDevicesDto,
    ) -> Result<String, AppError>;
}
