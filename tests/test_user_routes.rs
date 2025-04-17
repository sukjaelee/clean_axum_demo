use axum::extract::State;
use axum::http::Request;

use axum::Extension;
use axum::{body::Body, http::StatusCode, response::IntoResponse, Json};

use clean_axum_demo::app::create_router;
use clean_axum_demo::file::controllers::file_handler::delete_file;
use clean_axum_demo::shared::config::Config;
use clean_axum_demo::shared::error::AppError;
use clean_axum_demo::shared::jwt::Claims;
use clean_axum_demo::user::controllers::user_dto::{CreateUserMultipart, UpdateUser};
use clean_axum_demo::user::controllers::user_handlers::{
    delete_user, get_user_by_id, get_users, update_user,
};

mod test_helpers;

use test_helpers::{get_user_token, print_response, setup_test_db_state};
use tower::ServiceExt;

#[tokio::test]
async fn test_create_user() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let payload = CreateUserMultipart {
        username: "testuser-11".to_string(),
        email: "testuser-11@test.com".to_string(),
        modified_by: "00000000-0000-0000-0000-000000000001".to_string(),
        profile_picture: None,
    };

    let multipart_body = format!(
        "------XYZ\r\nContent-Disposition: form-data; name=\"username\"\r\n\r\n{}\r\n------XYZ\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\n{}\r\n------XYZ\r\nContent-Disposition: form-data; name=\"modified_by\"\r\n\r\n{}\r\n------XYZ--\r\n",
        payload.username, payload.email, payload.modified_by
    );

    let token = get_user_token().await.to_string();
    if token.is_empty() {
        return Err(AppError::TokenCreation);
    }

    let request = Request::builder()
        .method("POST")
        .uri("/users")
        .header("Authorization", token)
        .header("Content-Type", "multipart/form-data; boundary=----XYZ")
        .body(Body::from(multipart_body))
        .unwrap();

    // Pass this request into your app/router to extract Multipart in the handler.
    let config = Config::from_env().map_err(|_e| AppError::InternalError)?;

    let app = create_router(state.unwrap().pool, config.clone());
    let response = app.oneshot(request).await.unwrap();

    // Print the response body for debugging
    let (parts, body) = response.into_parts();
    print_response(body).await;
    // Check if the status code is OK
    assert_eq!(parts.status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_create_user_with_file() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let image_file = "cat.png";

    let payload = CreateUserMultipart {
        username: "testuser-12".to_string(),
        email: "testuser-12@example.com".to_string(),
        modified_by: "00000000-0000-0000-0000-000000000001".to_string(),
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

    let token = get_user_token().await.to_string();
    if token.is_empty() {
        return Err(AppError::TokenCreation);
    }

    let request = Request::builder()
        .method("POST")
        .uri("/users")
        .header("Authorization", token)
        .header("Content-Type", "multipart/form-data; boundary=----XYZ")
        .body(Body::from(multipart_body))
        .unwrap();

    // Pass the request into the app/router to let axum extract the multipart data
    let config = Config::from_env().map_err(|_e| AppError::InternalError)?;
    let app = create_router(state.unwrap().pool, config.clone());
    let response = app.oneshot(request).await.unwrap();

    // Print the response body for debugging
    let (parts, body) = response.into_parts();
    print_response(body).await;
    // Check if the status code is OK
    assert_eq!(parts.status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_get_users() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let claims = Claims {
        sub: "test001".to_string(),
        ..Default::default()
    };

    let response = get_users(State(state.unwrap()), Extension(claims))
        .await
        .into_response();

    let (parts, body) = response.into_parts();
    print_response(body).await;
    assert_eq!(parts.status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_get_user_by_id() -> Result<(), AppError> {
    let state = setup_test_db_state().await;
    let existent_id = "92fcf9ce-186f-11f0-8475-0242ac110002";

    let response = get_user_by_id(
        State(state.unwrap()),
        axum::extract::Path(existent_id.to_string()),
    )
    .await?
    .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_update_user() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let update_payload = UpdateUser {
        username: "updateduser-1".to_string(),
        email: "updated-1@test.com".to_string(),
        modified_by: "00000000-0000-0000-0000-000000000001".to_string(),
    };

    let existent_id = "00000000-0000-0000-0000-000000000001";

    let response = update_user(
        State(state.unwrap()),
        axum::extract::Path(existent_id.to_string()),
        Json(update_payload),
    )
    .await?
    .into_response();

    let (parts, body) = response.into_parts();
    print_response(body).await;
    assert_eq!(parts.status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_delete_user_not_found() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let non_existent_id = uuid::Uuid::new_v4();
    let response = delete_user(
        State(state.unwrap()),
        axum::extract::Path(non_existent_id.to_string()),
    )
    .await?
    .into_response();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
async fn test_delete_user() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let existent_id = "464f9f1a-1730-11f0-8475-0242ac110002";

    let response = delete_user(
        State(state.unwrap()),
        axum::extract::Path(existent_id.to_string()),
    )
    .await?
    .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_delete_user_file() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let existent_file_id = "f12f6300-18a9-11f0-8475-0242ac110002";

    let response = delete_file(
        State(state.unwrap()),
        axum::extract::Path(existent_file_id.to_string()),
    )
    .await?
    .into_response();

    let (parts, body) = response.into_parts();

    print_response(body).await;
    assert_eq!(parts.status, StatusCode::OK);

    Ok(())
}
