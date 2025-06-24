use crate::common::dto::RestApiResponse;
use crate::common::{app_state::AppState, error::AppError, jwt::Claims};

use crate::domains::device::dto::device_dto::{
    CreateDeviceDto, DeviceDto, UpdateDeviceDto, UpdateManyDevicesDto,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};

/// This function creates a router for getting a device by ID
/// It will return a device if found, otherwise it will return an error
#[utoipa::path(
    get,
    path = "/device/{id}",
    responses((status = 200, description = "Get device by ID", body = DeviceDto)),
    tag = "Devices"
)]
pub async fn get_device_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let device = state.device_service.get_device_by_id(id).await?;
    Ok(RestApiResponse::success(device))
}

/// This function creates a router for getting all devices
/// It will return a list of devices
#[utoipa::path(
    get,
    path = "/device",
    responses((status = 200, description = "List all devices", body = [DeviceDto])),
    tag = "Devices"
)]
pub async fn get_devices(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let devices = state.device_service.get_devices().await?;
    Ok(RestApiResponse::success(devices))
}

/// This function creates a router for creating a new device
/// It will create a new device in the database
/// It will return the created device
#[utoipa::path(
    post,
    path = "/device",
    request_body = CreateDeviceDto,
    responses((status = 200, description = "Create a new device", body = DeviceDto)),
    tag = "Devices"
)]
pub async fn create_device(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateDeviceDto>,
) -> Result<impl IntoResponse, AppError> {
    // Set the modified_by field to the current user's ID.
    let mut payload = payload;
    payload.modified_by = claims.sub.clone().to_string();

    let device = state.device_service.create_device(payload).await?;
    Ok(RestApiResponse::success(device))
}

/// This function creates a router for updating a device
/// It will update the device in the database
/// It will return the updated device
#[utoipa::path(
    put,
    path = "/device/{id}",
    request_body = UpdateDeviceDto,
    responses((status = 200, description = "Update device", body = DeviceDto)),
    tag = "Devices"
)]
pub async fn update_device(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<UpdateDeviceDto>,
) -> Result<impl IntoResponse, AppError> {
    // Set the modified_by field to the current user's ID.
    let mut payload = payload;
    payload.modified_by = claims.sub.clone().to_string();

    let device = state.device_service.update_device(id, payload).await?;
    Ok(RestApiResponse::success(device))
}

/// This function creates a router for deleting a device
/// It will delete the device from the database
/// It will return a message indicating the result of the operation
#[utoipa::path(
    delete,
    path = "/device/{id}",
    responses((status = 200, description = "Device deleted")),
    tag = "Devices"
)]
pub async fn delete_device(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let message = state.device_service.delete_device(id).await?;

    Ok(RestApiResponse::success_with_message(message, ()))
}

/// This function creates a router for batch updating devices
/// It will update multiple devices in the database
/// It will return a message indicating the result of the operation
#[utoipa::path(
    put,
    path = "/device/batch/{user_id}",
    request_body = UpdateManyDevicesDto,
    responses((status = 200, description = "Batch update devices")),
    tag = "Devices"
)]
pub async fn update_many_devices(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<String>,
    Json(payload): Json<UpdateManyDevicesDto>,
) -> Result<impl IntoResponse, AppError> {
    let modified_by = claims.sub.clone().to_string();

    let message = state
        .device_service
        .update_many_devices(user_id, modified_by, payload)
        .await?;

    Ok(RestApiResponse::success_with_message(message, ()))
}
