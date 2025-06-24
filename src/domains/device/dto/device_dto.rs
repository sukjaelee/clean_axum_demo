use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

use crate::domains::device::domain::model::{Device, DeviceOS, DeviceStatus};

#[derive(PartialEq, Debug, Deserialize, Serialize, ToSchema)]
pub struct DeviceDto {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub device_os: DeviceOS,
    pub status: DeviceStatus,
    #[serde(with = "crate::common::ts_format::option")]
    pub registered_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    #[serde(with = "crate::common::ts_format::option")]
    pub created_at: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
    #[serde(with = "crate::common::ts_format::option")]
    pub modified_at: Option<DateTime<Utc>>,
}

impl From<Device> for DeviceDto {
    fn from(device: Device) -> Self {
        Self {
            id: device.id,
            user_id: device.user_id,
            name: device.name,
            device_os: device.device_os,
            status: device.status,
            registered_at: device.registered_at,
            created_by: device.created_by,
            created_at: device.created_at,
            modified_by: device.modified_by,
            modified_at: device.modified_at,
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, serde::Serialize, ToSchema)]
pub struct CreateDeviceDto {
    pub name: String,
    pub user_id: String,
    pub device_os: DeviceOS,
    pub status: DeviceStatus,
    #[serde(with = "crate::common::ts_format::option")]
    pub registered_at: Option<DateTime<Utc>>,
    pub modified_by: String,
}

#[derive(PartialEq, Debug, Deserialize, serde::Serialize, ToSchema)]
pub struct UpdateDeviceDto {
    pub name: Option<String>,
    pub user_id: Option<String>,
    pub device_os: Option<DeviceOS>,
    pub status: Option<DeviceStatus>,
    #[serde(with = "crate::common::ts_format::option")]
    pub registered_at: Option<DateTime<Utc>>,
    pub modified_by: String,
}

#[derive(Debug, Deserialize, serde::Serialize, ToSchema)]
pub struct UpdateManyDevicesDto {
    pub devices: Vec<UpdateDeviceDtoWithIdDto>,
}

#[derive(PartialEq, Debug, Deserialize, serde::Serialize, ToSchema)]
pub struct UpdateDeviceDtoWithIdDto {
    pub id: Option<String>,
    pub name: String,
    pub device_os: DeviceOS,
    pub status: DeviceStatus,
}
