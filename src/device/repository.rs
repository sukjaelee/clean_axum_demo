use super::dto::*;
use super::model::Device;

use async_trait::async_trait;
use sqlx::{MySql, Pool, Transaction};

#[async_trait]
pub trait DeviceRepository: Send + Sync {
    async fn find_all(&self, pool: Pool<MySql>) -> Result<Vec<Device>, sqlx::Error>;
    async fn find_by_id(
        &self,
        pool: Pool<MySql>,
        id: String,
    ) -> Result<Option<Device>, sqlx::Error>;
    async fn create(
        &self,
        tx: &mut Transaction<'_, MySql>,
        device: CreateDevice,
    ) -> Result<Device, sqlx::Error>;
    async fn update(
        &self,
        tx: &mut Transaction<'_, MySql>,
        id: String,
        device: UpdateDevice,
    ) -> Result<Option<Device>, sqlx::Error>;
    async fn update_many(
        &self,
        tx: &mut Transaction<'_, MySql>,
        user_id: String,
        modified_by: String,
        update_devices: UpdateManyDevices,
    ) -> Result<(), sqlx::Error>;
    async fn delete(
        &self,
        tx: &mut Transaction<'_, MySql>,
        id: String,
    ) -> Result<bool, sqlx::Error>;
}
