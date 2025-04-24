use axum::http::{Method, StatusCode};

use clean_axum_demo::{
    common::dto::RestApiResponse,
    device::{
        domain::model::{DeviceOS, DeviceStatus},
        dto::{
            CreateDeviceDto, DeviceDto, UpdateDeviceDto, UpdateDeviceDtoWithIdDto,
            UpdateManyDevicesDto,
        },
    },
};

use uuid::Uuid;
mod test_helpers;
use test_helpers::{
    deserialize_json_body, request_with_auth, request_with_auth_and_body, TEST_USER_ID,
};

async fn create_test_device() -> DeviceDto {
    let name = format!("test-device-{}", Uuid::new_v4()).to_string();

    let payload = CreateDeviceDto {
        name,
        user_id: TEST_USER_ID.to_string(),
        device_os: DeviceOS::Android,
        status: DeviceStatus::Active,
        modified_by: TEST_USER_ID.to_string(),
    };

    let response = request_with_auth_and_body(Method::POST, "/device", &payload);
    let (parts, body) = response.await.into_parts();
    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<DeviceDto> = deserialize_json_body(body).await.unwrap();
    response_body.0.data.unwrap()
}

#[tokio::test]
async fn test_create_device() {
    let name = format!("test-device-{}", Uuid::new_v4()).to_string();

    let payload = CreateDeviceDto {
        name,
        user_id: TEST_USER_ID.to_string(),
        device_os: DeviceOS::Android,
        status: DeviceStatus::Active,
        modified_by: TEST_USER_ID.to_string(),
    };

    let response = request_with_auth_and_body(Method::POST, "/device", &payload);

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<DeviceDto> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    let device_dto = response_body.0.data.unwrap();

    assert_eq!(device_dto.name, payload.name);
    assert_eq!(device_dto.user_id, payload.user_id);
    assert_eq!(device_dto.device_os, payload.device_os);
    assert_eq!(device_dto.status, payload.status);
    assert_ne!(device_dto.modified_by, Some(payload.modified_by));
}

async fn get_devices() -> Vec<DeviceDto> {
    let response = request_with_auth(Method::GET, "/device");

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<Vec<DeviceDto>> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);

    response_body.0.data.unwrap()
}

#[tokio::test]
async fn test_get_devices() {
    let devices = get_devices().await;
    // println!("devices: {:?}", devices);
    assert!(!devices.is_empty());
}

#[tokio::test]
async fn test_get_device_by_id() {
    let device = create_test_device().await;
    let existent_id = &device.id;

    let url = format!("/device/{}", existent_id);
    let response = request_with_auth(Method::GET, url.as_str());

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<DeviceDto> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);

    let response_device = response_body.0.data.unwrap();

    assert_eq!(response_device.id, *existent_id);
    assert_eq!(response_device.name, device.name);
    assert_eq!(response_device.user_id, device.user_id);
    assert_eq!(response_device.device_os, device.device_os);
    assert_eq!(response_device.status, device.status);
}

#[tokio::test]
async fn test_update_device() {
    let existent_device = &create_test_device().await;

    let name = format!("update-device-{}", Uuid::new_v4()).to_string();

    let payload = UpdateDeviceDto {
        name: Some(name),
        user_id: Some(existent_device.user_id.clone()),
        device_os: Some(DeviceOS::IOS),
        status: Some(DeviceStatus::Decommissioned),
        modified_by: existent_device.modified_by.clone().unwrap_or_default(),
    };

    let existent_id = existent_device.id.clone();

    let url = format!("/device/{}", existent_id);

    let response = request_with_auth_and_body(Method::PUT, url.as_str(), &payload);

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<DeviceDto> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);

    let response_device = response_body.0.data.unwrap();

    assert_eq!(response_device.id, *existent_id);
    assert_eq!(Some(response_device.name), payload.name);
    assert_eq!(Some(response_device.user_id), payload.user_id);
    assert_eq!(Some(response_device.device_os), payload.device_os);
    assert_eq!(Some(response_device.status), payload.status);
}

#[tokio::test]
async fn test_delete_device_not_found() {
    let non_existent_id = uuid::Uuid::new_v4();

    let url = format!("/device/{}", non_existent_id);
    let response = request_with_auth(Method::DELETE, url.as_str());

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::NOT_FOUND);

    let response_body: RestApiResponse<()> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::NOT_FOUND);
    // println!("response_body.0.status: {:?}", response_body.0.status);
    // println!("response_body.0.message: {:?}", response_body.0.message);
}

#[tokio::test]
async fn test_delete_device() {
    let devices = get_devices().await;
    let existent_device = &devices[0];
    let existent_id = existent_device.id.clone();

    let url = format!("/device/{}", existent_id);
    let response = request_with_auth(Method::DELETE, url.as_str());

    let (parts, body) = response.await.into_parts();
    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<()> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    // println!("response_body.0.status: {:?}", response_body.0.status);
    // println!("response_body.0.message: {:?}", response_body.0.message);
}

#[tokio::test]
async fn test_update_many_devices() {
    let devices = get_devices().await;
    let existent_device = &devices[0];

    let user_id = TEST_USER_ID.to_string();

    let name1 = format!("many-update-device-{}", Uuid::new_v4()).to_string();
    let name2 = format!("many-update-in-device-{}", Uuid::new_v4()).to_string();

    let payload = UpdateManyDevicesDto {
        devices: vec![
            UpdateDeviceDtoWithIdDto {
                id: Some(existent_device.id.clone()),
                name: name1.clone(),
                device_os: DeviceOS::IOS,
                status: DeviceStatus::Blocked,
            },
            UpdateDeviceDtoWithIdDto {
                id: None,
                name: name2.clone(),
                device_os: DeviceOS::Android,
                status: DeviceStatus::Pending,
            },
        ],
    };

    let url = format!("/device/batch/{}", user_id);

    let response = request_with_auth_and_body(Method::PUT, url.as_str(), &payload);

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<()> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    // println!("response_body.0.status: {:?}", response_body.0.status);
    // println!("response_body.0.message: {:?}", response_body.0.message);
}
