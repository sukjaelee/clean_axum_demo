use axum::http::{Method, StatusCode};

mod test_helpers;

use test_helpers::{request, request_with_auth};

#[tokio::test]
async fn test_public_assets() {
    let response = request(Method::GET, "/assets/public/images.jpeg");

    let (parts, _) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);
}

#[tokio::test]
async fn test_private_assets_without_auth() {
    let response = request(Method::GET, "/assets/private/profile_picture/images.jpeg");

    let (parts, _) = response.await.into_parts();
    // println!("parts.status: {:?}", parts.status);
    assert_eq!(parts.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_private_assets_with_auth() {
    let response = request_with_auth(Method::GET, "/assets/private/profile_picture/images.jpeg");

    let (parts, _) = response.await.into_parts();
    // println!("parts.status: {:?}", parts.status);
    assert_eq!(parts.status, StatusCode::OK);
}
