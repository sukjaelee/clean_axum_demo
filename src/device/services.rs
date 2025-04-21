use super::dto::*;

use crate::common::error::AppError;
use crate::device::{queries::DeviceRepo, repository::DeviceRepository};

use sqlx::MySqlPool;
use std::sync::Arc;

/// Service struct for handling device-related operations
/// such as creating, updating, deleting, and fetching devices.
/// It uses a repository pattern to abstract the data access layer.
#[derive(Clone)]
pub struct DeviceService {
    pool: MySqlPool,
    repo: Arc<dyn DeviceRepository + Send + Sync>,
}

/// Implementation of the DeviceService struct
impl DeviceService {
    /// Creates a new instance of DeviceService.
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            pool,
            repo: Arc::new(DeviceRepo {}),
        }
    }

    /// get device by id
    pub async fn get_device_by_id(&self, id: String) -> Result<DeviceDto, AppError> {
        match self.repo.find_by_id(self.pool.clone(), id).await {
            Ok(Some(device)) => Ok(DeviceDto::from(device)),
            Ok(None) => Err(AppError::NotFound("Device not found".into())),
            Err(err) => {
                tracing::error!("Error fetching device: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// get devices
    pub async fn get_devices(&self) -> Result<Vec<DeviceDto>, AppError> {
        match self.repo.find_all(self.pool.clone()).await {
            Ok(devices) => {
                let device_dtos: Vec<DeviceDto> = devices.into_iter().map(Into::into).collect();
                Ok(device_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching devices: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// create device
    pub async fn create_device(&self, payload: CreateDeviceDto) -> Result<DeviceDto, AppError> {
        let mut tx = self.pool.begin().await?;
        match self.repo.create(&mut tx, payload).await {
            Ok(device) => {
                tx.commit().await?;
                Ok(DeviceDto::from(device))
            }
            Err(err) => {
                tracing::error!("Error creating device: {err}");
                tx.rollback().await?;
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// update device
    pub async fn update_device(
        &self,
        id: String,
        payload: UpdateDeviceDto,
    ) -> Result<DeviceDto, AppError> {
        let mut tx = self.pool.begin().await?;
        match self.repo.update(&mut tx, id, payload).await {
            Ok(Some(device)) => {
                tx.commit().await?;
                Ok(DeviceDto::from(device))
            }
            Ok(None) => {
                tx.rollback().await?;
                Err(AppError::NotFound("Device not found".into()))
            }
            Err(err) => {
                tracing::error!("Error updating device: {err}");
                tx.rollback().await?;
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// delete device
    pub async fn delete_device(&self, id: String) -> Result<String, AppError> {
        let mut tx = self.pool.begin().await?;
        match self.repo.delete(&mut tx, id).await {
            Ok(true) => {
                tx.commit().await?;
                Ok("Device deleted".into())
            }
            Ok(false) => {
                tx.rollback().await?;
                Err(AppError::NotFound("Device not found".into()))
            }
            Err(err) => {
                tracing::error!("Error deleting device: {err}");
                tx.rollback().await?;
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// batch update device
    pub async fn update_many_devices(
        &self,
        user_id: String,
        modified_by: String,
        payload: UpdateManyDevicesDto,
    ) -> Result<String, AppError> {
        let mut tx = self.pool.begin().await?;
        match self
            .repo
            .update_many(&mut tx, user_id, modified_by, payload)
            .await
        {
            Ok(()) => {
                tx.commit().await?;
                Ok("Devices updated".into())
            }
            Err(err) => {
                tracing::error!("Error batch update device: {err}");
                tx.rollback().await?;
                Err(AppError::DatabaseError(err))
            }
        }
    }
}
