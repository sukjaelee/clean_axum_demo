use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use clean_axum_demo::{
    device::{
        dto::{CreateDevice, UpdateDevice, UpdateDeviceWithId, UpdateManyDevices},
        handlers::{
            create_device, delete_device, get_device_by_id, get_devices, update_device,
            update_many_devices,
        },
        model::{DeviceOS, DeviceStatus},
    },
    shared::{error::AppError, jwt::Claims},
};
mod test_helpers;
use test_helpers::setup_test_db_state;

#[tokio::test]
async fn test_create_device() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let payload = CreateDevice {
        name: "Integration 테스트 Device4".to_string(),
        user_id: "00000000-0000-0000-0000-000000000001".to_string(),
        device_os: DeviceOS::Android,
        status: DeviceStatus::Active,
        modified_by: "00000000-0000-0000-0000-000000000001".to_string(),
    };

    let claims = Claims {
        sub: "00000000-0000-0000-0000-000000000021".to_string(),
        ..Default::default()
    };

    let response = create_device(State(state.unwrap()), Extension(claims), Json(payload)).await?;
    let status = response.into_response().status();
    assert_eq!(status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_get_devices() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let response = get_devices(State(state.unwrap())).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_get_device_by_id() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let existent_id = "b0a994dd-15b2-11f0-8457-0242ac110002";

    let response = get_device_by_id(
        State(state.unwrap()),
        axum::extract::Path(existent_id.to_string()),
    )
    .await?
    .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_update_device() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let update_payload = UpdateDevice {
        name: Some("Updated Name2-1".to_string()),
        user_id: Some("00000000-0000-0000-0000-000000000001".to_string()),
        device_os: Some(DeviceOS::IOS),
        status: Some(DeviceStatus::Decommissioned),
        modified_by: "00000000-0000-0000-0000-000000000001".to_string(),
    };

    let existent_id = "b0a99b38-15b2-11f0-8457-0242ac110002";

    let claims = Claims {
        sub: "00000000-0000-0000-0000-000000000021".to_string(),
        ..Default::default()
    };

    let response = update_device(
        State(state.unwrap()),
        Extension(claims),
        axum::extract::Path(existent_id.to_string()),
        Json(update_payload),
    )
    .await?
    .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_delete_device_not_found() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let non_existent_id = uuid::Uuid::new_v4();
    let response = delete_device(
        State(state.unwrap()),
        axum::extract::Path(non_existent_id.to_string()),
    )
    .await?
    .into_response();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
async fn test_delete_device() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let existent_id = "b0a994dd-15b2-11f0-8457-0242ac110002";

    let response = delete_device(
        State(state.unwrap()),
        axum::extract::Path(existent_id.to_string()),
    )
    .await?
    .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_update_many_devices() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let user_id = "00000000-0000-0000-0000-000000000001";

    let payload = UpdateManyDevices {
        devices: vec![
            UpdateDeviceWithId {
                id: Some("b0a99bdc-15b2-11f0-8457-0242ac110002".to_string()),
                name: "Bulk Device 1-1".to_string(),
                device_os: DeviceOS::IOS,
                status: DeviceStatus::Blocked,
            },
            UpdateDeviceWithId {
                id: None,
                name: "Bulk Device 2-1".to_string(),
                device_os: DeviceOS::Android,
                status: DeviceStatus::Pending,
            },
        ],
    };

    let claims = Claims {
        sub: "00000000-0000-0000-0000-000000000021".to_string(),
        ..Default::default()
    };

    let response = update_many_devices(
        State(state.unwrap()),
        Extension(claims),
        Path(user_id.to_string()),
        Json(payload),
    )
    .await?
    .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
