//! Domain model definitions for device-related entities.
//! This includes enums for device status and OS, as well as the core `Device` struct.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    decode::Decode,
    postgres::{PgTypeInfo, PgValueRef},
    FromRow, Postgres, Type,
};
use std::{fmt, str::FromStr};
use utoipa::ToSchema;

use crate::common::error::AppError;

/// Enum representing the possible statuses of a device in the system.
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

impl<'r> Decode<'r, Postgres> for DeviceStatus {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(DeviceStatus::from_str(s)?)
    }
}

impl Type<Postgres> for DeviceStatus {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

#[allow(clippy::upper_case_acronyms)]
/// Enum representing the supported operating systems of a device.
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

impl<'r> Decode<'r, Postgres> for DeviceOS {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(DeviceOS::from_str(s)?)
    }
}

impl Type<Postgres> for DeviceOS {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

/// Domain model representing a device entity.
#[derive(Debug, Clone, FromRow)]
pub struct Device {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub device_os: DeviceOS,
    pub status: DeviceStatus,
    pub registered_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
    pub modified_at: Option<DateTime<Utc>>,
}
