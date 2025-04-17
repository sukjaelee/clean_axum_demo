use crate::device::controllers::device_dto::{CreateDevice, UpdateDevice};
use crate::device::model::device_model::Device;
use crate::shared::app_state::AppState;
use crate::shared::error::AppError;
use axum::extract::{Path, State};
use axum::{response::IntoResponse, Json};
use serde_json::json;

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;

use super::device_dto::UpdateManyDevices;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_device_by_id,
        get_devices,
        create_device,
        update_device,
        update_many_devices,
        delete_device,
    ),
    components(schemas(Device, CreateDevice, UpdateDevice)),
    tags(
        (name = "Devices", description = "Device management endpoints")
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&DeviceApiDoc)
)]

/// This struct is used to generate OpenAPI documentation for the device routes.
pub struct DeviceApiDoc;

impl utoipa::Modify for DeviceApiDoc {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("Input your `<your‑jwt>`"))
                    .build(),
            ),
        )
    }
}

#[utoipa::path(
    get,
    path = "/devices/{id}",
    responses((status = 200, description = "Get device by ID", body = Device)),
    tag = "Devices"
)]
pub async fn get_device_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match state.device_repo.find_by_id(state.pool, id).await {
        Ok(Some(device)) => Ok(Json(json!({ "device": device })).into_response()),
        Ok(None) => Ok((AppError::NotFound("Device not found".into())).into_response()),
        Err(err) => {
            tracing::error!("Error fetching device: {err}");
            Err(AppError::DatabaseError(err))
        }
    }
}

#[utoipa::path(
    get,
    path = "/devices",
    responses((status = 200, description = "List all devices", body = [Device])),
    tag = "Devices"
)]
pub async fn get_devices(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    match state.device_repo.find_all(state.pool).await {
        Ok(devices) => Ok(Json(json!({ "devices": devices })).into_response()),
        Err(err) => {
            tracing::error!("Error fetching devices: {err}");
            Err(AppError::DatabaseError(err))
        }
    }
}

#[utoipa::path(
    post,
    path = "/devices",
    request_body = CreateDevice,
    responses((status = 200, description = "Create a new device", body = Device)),
    tag = "Devices"
)]
pub async fn create_device(
    State(state): State<AppState>,
    Json(payload): Json<CreateDevice>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.pool.begin().await?;

    match state.device_repo.create(&mut tx, payload).await {
        Ok(device) => {
            tx.commit().await?; // Commit the transaction
            Ok(Json(json!({ "device": device })).into_response())
        }
        Err(err) => {
            tracing::error!("Error creating device: {err}");
            tx.rollback().await?; // Rollback the transaction
            Err(AppError::DatabaseError(err))
        }
    }
}

#[utoipa::path(
    put,
    path = "/devices/{id}",
    request_body = UpdateDevice,
    responses((status = 200, description = "Update device", body = Device)),
    tag = "Devices"
)]
pub async fn update_device(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<UpdateDevice>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.pool.begin().await?;

    match state.device_repo.update(&mut tx, id, payload).await {
        Ok(Some(device)) => {
            tx.commit().await?; // Commit the transaction
            Ok(Json(json!({ "device": device })).into_response())
        }
        Ok(None) => {
            tx.rollback().await?; // Rollback the transaction
            Ok((AppError::NotFound("Device not found".into())).into_response())
        }
        Err(err) => {
            tracing::error!("Error updating device: {err}");
            tx.rollback().await?; // Rollback the transaction
            Err(AppError::DatabaseError(err))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/devices/{id}",
    responses((status = 200, description = "Device deleted")),
    tag = "Devices"
)]
pub async fn delete_device(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.pool.begin().await?;

    match state.device_repo.delete(&mut tx, id).await {
        Ok(true) => {
            tx.commit().await?; // Commit the transaction
            Ok((axum::http::StatusCode::OK, "Device deleted").into_response())
        }
        Ok(false) => {
            tx.rollback().await?; // Rollback the transaction
            Ok((AppError::NotFound("Device not found".into())).into_response())
        }
        Err(err) => {
            tracing::error!("Error deleting device: {err}");
            tx.rollback().await?; // Rollback the transaction
            Err(AppError::DatabaseError(err))
        }
    }
}

#[utoipa::path(
    put,
    path = "/devices/batch/{user_id}",
    request_body = UpdateManyDevices,
    responses((status = 200, description = "Batch update devices")),
    tag = "Devices"
)]
pub async fn update_many_devices(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(payload): Json<UpdateManyDevices>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.pool.begin().await?;

    match state
        .device_repo
        .update_many(&mut tx, user_id, payload)
        .await
    {
        Ok(()) => {
            tx.commit().await?; // Commit the transaction
            Ok((axum::http::StatusCode::OK, "Devices updated").into_response())
        }
        Err(err) => {
            tracing::error!("Error creating device: {err}");
            tx.rollback().await?; // Rollback the transaction
            Err(AppError::DatabaseError(err))
        }
    }
}
