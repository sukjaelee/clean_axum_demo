use axum::http::{Method, StatusCode};

use clean_axum_demo::{
    common::{dto::RestApiResponse, error::AppError},
    user::dto::{CreateUserMultipartDto, UpdateUserDto, UserDto},
};

mod test_helpers;

use test_helpers::{
    deserialize_json_body, request_with_auth, request_with_auth_and_body,
    request_with_auth_and_multipart, TEST_USER_ID,
};

async fn create_user() -> Result<(CreateUserMultipartDto, UserDto), AppError> {
    let username = format!("testuser-{}", uuid::Uuid::new_v4()).to_string();
    let email = format!("{}@test.com", username).to_string();

    let payload = CreateUserMultipartDto {
        username,
        email,
        modified_by: TEST_USER_ID.to_string(),
        profile_picture: None,
    };

    let multipart_body = format!(
        "------XYZ\r\nContent-Disposition: form-data; name=\"username\"\r\n\r\n{}\r\n------XYZ\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\n{}\r\n------XYZ\r\nContent-Disposition: form-data; name=\"modified_by\"\r\n\r\n{}\r\n------XYZ--\r\n",
        payload.username, payload.email, payload.modified_by
    ).as_bytes().to_vec();

    let response = request_with_auth_and_multipart(Method::POST, "/user", multipart_body);

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<UserDto> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    let user_dto = response_body.0.data.unwrap();

    Ok((payload, user_dto))
}

#[tokio::test]
async fn test_create_user() {
    let created = create_user().await.expect("Failed to create user");

    let payload = created.0;
    let user_dto = created.1;

    assert!(!user_dto.id.is_empty());
    assert_eq!(user_dto.username, payload.username.clone());
    assert_eq!(user_dto.email, Some(payload.email.clone()));
    assert_ne!(user_dto.modified_by, Some(payload.modified_by.clone()));
    assert_eq!(user_dto.origin_file_name, None);
    assert!(user_dto.file_id.is_none());
}

async fn create_user_with_file() -> Result<(CreateUserMultipartDto, UserDto, String), AppError> {
    let username = format!("testuser-{}", uuid::Uuid::new_v4()).to_string();
    let email = format!("{}@test.com", username).to_string();

    let image_file = "cat.png";

    let payload = CreateUserMultipartDto {
        username,
        email,
        modified_by: TEST_USER_ID.to_string(),
        // Indicate the file name being uploaded
        profile_picture: Some(image_file.to_string()),
    };

    // Read the image file from the test/asset/ directory
    let file_path = format!("tests/asset/{}", image_file);
    let file_bytes = std::fs::read(file_path)
        .expect(format!("Failed to read {} from tests/asset/", image_file).as_str());

    // Build the multipart body as a byte vector (Vec<u8>)
    let mut multipart_body = Vec::new();
    use std::io::Write;
    // Add the username part
    write!(
        &mut multipart_body,
        "------XYZ\r\nContent-Disposition: form-data; name=\"username\"\r\n\r\n{}\r\n",
        payload.username
    )
    .unwrap();
    // Add the email part
    write!(
        &mut multipart_body,
        "------XYZ\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\n{}\r\n",
        payload.email
    )
    .unwrap();
    // Add the modified_by part
    write!(
        &mut multipart_body,
        "------XYZ\r\nContent-Disposition: form-data; name=\"modified_by\"\r\n\r\n{}\r\n",
        payload.modified_by
    )
    .unwrap();
    // Add the file part for profile_picture
    write!(
        &mut multipart_body,
        "------XYZ\r\nContent-Disposition: form-data; name=\"profile_picture\"; filename=\"{}\"\r\nContent-Type: image/png\r\n\r\n",
        image_file
    ).unwrap();
    multipart_body.extend_from_slice(&file_bytes);
    write!(&mut multipart_body, "\r\n").unwrap();
    // Add the final boundary
    write!(&mut multipart_body, "------XYZ--\r\n").unwrap();

    let response = request_with_auth_and_multipart(Method::POST, "/user", multipart_body);

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<UserDto> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    let user_dto = response_body.0.data.unwrap();

    Ok((payload, user_dto, image_file.to_string()))
}

#[tokio::test]
async fn test_create_user_with_file() {
    let created = create_user_with_file()
        .await
        .expect("Failed to create user with file");

    let payload = created.0;
    let user_dto = created.1;
    let image_file = created.2;

    assert!(!user_dto.id.is_empty());
    assert_eq!(user_dto.username, payload.username.clone());
    assert_eq!(user_dto.email, Some(payload.email.clone()));
    assert_ne!(user_dto.modified_by, Some(payload.modified_by.clone()));
    assert_eq!(user_dto.origin_file_name, Some(image_file.to_string()));
    assert!(!user_dto.file_id.clone().unwrap_or_default().is_empty());
}

async fn get_users() -> Vec<UserDto> {
    let response = request_with_auth(Method::GET, "/user");

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<Vec<UserDto>> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    let user_dtos = response_body.0.data.unwrap();

    user_dtos
}

#[tokio::test]
async fn test_get_users() {
    let user_dtos: Vec<UserDto> = get_users().await;
    // println!("user_dtos: {:?}", user_dtos);
    assert!(!user_dtos.is_empty());
}

#[tokio::test]
async fn test_get_user_by_id() {
    let users = get_users().await;
    let existent_id = &users[0].id;

    let url = format!("/user/{}", existent_id);
    let response = request_with_auth(Method::GET, url.as_str());

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<UserDto> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    let user_dto = response_body.0.data.unwrap();

    assert_eq!(user_dto.id, *existent_id);
    assert_eq!(user_dto.username, users[0].username);
    assert_eq!(user_dto.email, users[0].email);
    assert_eq!(user_dto.created_by, users[0].created_by);
    assert_eq!(user_dto.created_at, users[0].created_at);
    assert_eq!(user_dto.modified_by, users[0].modified_by);
    assert_eq!(user_dto.modified_at, users[0].modified_at);
    assert_eq!(user_dto.file_id, users[0].file_id);
    assert_eq!(user_dto.origin_file_name, users[0].origin_file_name);
}

#[tokio::test]
async fn test_update_user() {
    let users = get_users().await;
    let existent_id = &users[0].id;

    let username = format!("update-testuser-{}", uuid::Uuid::new_v4()).to_string();
    let email = format!("{}@test.com", username).to_string();

    let payload = UpdateUserDto {
        username,
        email,
        modified_by: TEST_USER_ID.to_string(),
    };

    let url = format!("/user/{}", existent_id);

    let response = request_with_auth_and_body(Method::PUT, url.as_str(), &payload);

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<UserDto> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    let user_dto = response_body.0.data.unwrap();

    assert_eq!(user_dto.id, *existent_id);
    assert_eq!(user_dto.username, payload.username);
    assert_eq!(user_dto.email, Some(payload.email));
}

#[tokio::test]
async fn test_delete_user_not_found() {
    let non_existent_id = uuid::Uuid::new_v4();

    let url = format!("/user/{}", non_existent_id);
    let response = request_with_auth(Method::DELETE, url.as_str());

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::NOT_FOUND);

    let response_body: RestApiResponse<()> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::NOT_FOUND);
    // println!("response_body.0.status: {:?}", response_body.0.status);
    // println!("response_body.0.message: {:?}", response_body.0.message);
}

#[tokio::test]
async fn test_delete_user() {
    let created = create_user()
        .await
        .expect("Failed to create user for deletion");

    let user = created.1;

    let url = format!("/user/{}", user.id);

    let response = request_with_auth(Method::DELETE, url.as_str());

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<()> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    // println!("response_body.0.status: {:?}", response_body.0.status);
    // println!("response_body.0.message: {:?}", response_body.0.message);
}

#[tokio::test]
async fn test_delete_user_file() {
    let created = create_user_with_file()
        .await
        .expect("Failed to create user with file for deletion");
    let user_dto = created.1;
    let file_id = user_dto.file_id.clone().unwrap_or_default();

    let url = format!("/file/{}", file_id);

    let response = request_with_auth(Method::DELETE, url.as_str());

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<()> = deserialize_json_body(body).await.unwrap();

    assert_eq!(response_body.0.status, StatusCode::OK);
    // println!("response_body.0.status: {:?}", response_body.0.status);
    // println!("response_body.0.message: {:?}", response_body.0.message);
}
