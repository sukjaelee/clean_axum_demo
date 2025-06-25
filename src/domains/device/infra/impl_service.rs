use crate::{
    common::error::AppError,
    domains::device::{
        domain::{repository::DeviceRepository, service::DeviceServiceTrait},
        dto::device_dto::{CreateDeviceDto, DeviceDto, UpdateDeviceDto, UpdateManyDevicesDto},
        infra::impl_repository::DeviceRepo,
    },
};

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

/// Service struct for handling device-related operations
/// such as creating, updating, deleting, and fetching devices.
/// It uses a repository pattern to abstract the data access layer.
#[derive(Clone)]
pub struct DeviceService {
    pool: PgPool,
    repo: Arc<dyn DeviceRepository + Send + Sync>,
}

/// Implementation of the DeviceService struct
#[async_trait]
impl DeviceServiceTrait for DeviceService {
    /// constructor for the service.
    fn create_service(pool: PgPool) -> Arc<dyn DeviceServiceTrait> {
        Arc::new(Self {
            pool,
            repo: Arc::new(DeviceRepo {}),
        })
    }

    /// get device by id
    async fn get_device_by_id(&self, id: String) -> Result<DeviceDto, AppError> {
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
    async fn get_devices(&self) -> Result<Vec<DeviceDto>, AppError> {
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
    async fn create_device(&self, payload: CreateDeviceDto) -> Result<DeviceDto, AppError> {
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
    async fn update_device(
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
    async fn delete_device(&self, id: String) -> Result<String, AppError> {
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
    async fn update_many_devices(
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
