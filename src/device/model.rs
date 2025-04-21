use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use time::OffsetDateTime;
use utoipa::ToSchema;

use crate::common::error::AppError;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum DeviceStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "decommissioned")]
    Decommissioned,
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DeviceStatus::Active => "active",
            DeviceStatus::Inactive => "inactive",
            DeviceStatus::Pending => "pending",
            DeviceStatus::Blocked => "blocked",
            DeviceStatus::Decommissioned => "decommissioned",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for DeviceStatus {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(DeviceStatus::Active),
            "inactive" => Ok(DeviceStatus::Inactive),
            "pending" => Ok(DeviceStatus::Pending),
            "blocked" => Ok(DeviceStatus::Blocked),
            "decommissioned" => Ok(DeviceStatus::Decommissioned),
            _ => Err(AppError::ValidationError(format!("Invalid status: {s}"))),
        }
    }
}

impl From<String> for DeviceStatus {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap_or_else(|_| panic!("Invalid device status: {}", s))
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum DeviceOS {
    #[serde(rename = "Android")]
    Android,
    #[serde(rename = "iOS")]
    IOS,
}

impl fmt::Display for DeviceOS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DeviceOS::Android => "Android",
            DeviceOS::IOS => "iOS",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for DeviceOS {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Android" => Ok(DeviceOS::Android),
            "iOS" => Ok(DeviceOS::IOS),
            _ => Err(AppError::ValidationError(format!("Invalid device_os: {s}"))),
        }
    }
}

impl From<String> for DeviceOS {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap_or_else(|_| panic!("Invalid device OS: {}", s))
    }
}

#[derive(Debug, Clone)]
pub struct Device {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub device_os: DeviceOS,
    pub status: DeviceStatus,
    pub registered_at: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub modified_by: Option<String>,
    pub modified_at: Option<OffsetDateTime>,
}
