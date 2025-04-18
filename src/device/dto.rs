use serde::Deserialize;
use utoipa::ToSchema;

use super::model::{DeviceOS, DeviceStatus};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDevice {
    pub name: String,
    pub user_id: String,
    pub device_os: DeviceOS,
    pub status: DeviceStatus,
    pub modified_by: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDevice {
    pub name: Option<String>,
    pub user_id: Option<String>,
    pub device_os: Option<DeviceOS>,
    pub status: Option<DeviceStatus>,
    pub modified_by: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateManyDevices {
    pub devices: Vec<UpdateDeviceWithId>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDeviceWithId {
    pub id: Option<String>,
    pub name: String,
    pub device_os: DeviceOS,
    pub status: DeviceStatus,
}
