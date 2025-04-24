use std::sync::Once;

use axum::{
    body::Body,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method, Request, Response, StatusCode,
    },
    Router,
};

use dotenv::from_filename;
use http_body_util::BodyExt;

use clean_axum_demo::{
    app::create_router,
    common::{
        bootstrap::build_app_state,
        config::Config,
        dto::RestApiResponse,
        jwt::{AuthBody, AuthPayload},
    },
};
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySql, Pool,
};
use tower::ServiceExt;

use std::str::FromStr;

static INIT: Once = Once::new();

/// Constants for test client credentials
/// These are used to authenticate the test client
#[allow(dead_code)]
pub const TEST_CLIENT_ID: &str = "apitest01";
#[allow(dead_code)]
pub const TEST_CLIENT_SECRET: &str = "test_password";

#[allow(dead_code)]
pub const TEST_USER_ID: &str = "00000000-0000-0000-0000-000000000001";

/// Helper function to load environment variables from .env.test file
fn load_test_env() {
    INIT.call_once(|| {
        from_filename(".env.test").expect("Failed to load .env.test");
    });
}

/// Helper function to set up the test database state
pub async fn setup_test_db() -> Result<Pool<MySql>, Box<dyn std::error::Error>> {
    load_test_env();
    let config = Config::from_env()?;

    // Create connection options
    let connect_options = MySqlConnectOptions::from_str(&config.database_url)
        .map_err(|e| {
            tracing::error!("Failed to parse database URL: {}", e);
            e
        })?
        .charset(&config.database_charset)
        .clone();

    // Avoid using problematic timezone settings unless absolutely required
    // If you must set timezone, do it in SQL after connect

    let pool = MySqlPoolOptions::new()
        .max_connections(config.database_max_connections)
        .min_connections(config.database_min_connections)
        .connect_with(connect_options)
        .await?;

    // Optional: set timezone in session
    sqlx::query(&format!("SET time_zone = '{}'", config.database_time_zone))
        .execute(&pool)
        .await?;

    Ok(pool)
}

/// Helper function to create a test router
pub async fn create_test_router() -> Router {
    let pool = setup_test_db().await.unwrap();
    let config = Config::from_env().unwrap();
    let state = build_app_state(pool, config.clone());
    let app = create_router(state);

    app
}

/// Helper function gets the authentication token
/// for the test client
/// This function is used to authenticate the test client
#[allow(dead_code)]
async fn get_authentication_token() -> String {
    let payload = AuthPayload {
        client_id: TEST_CLIENT_ID.to_string(),
        client_secret: TEST_CLIENT_SECRET.to_string(),
    };

    let response = request_with_body(Method::POST, "/auth/login", &payload);

    let (parts, body) = response.await.into_parts();

    assert_eq!(parts.status, StatusCode::OK);

    let response_body: RestApiResponse<AuthBody> = deserialize_json_body(body).await.unwrap();
    let auth_body = response_body.0.data.unwrap();
    let token = format!("{} {}", auth_body.token_type, auth_body.access_token);
    token
}

/// Helper function to deserialize the body of a request into a specific type
pub async fn deserialize_json_body<T: serde::de::DeserializeOwned>(
    body: Body,
) -> Result<T, Box<dyn std::error::Error>> {
    let bytes = body
        .collect()
        .await
        .map_err(|e| {
            tracing::error!("Failed to collect response body: {}", e);
            e
        })?
        .to_bytes();

    if bytes.is_empty() {
        return Err(("Empty response body").into());
    }

    // Debugging output
    // Uncomment the following lines to print the response body
    // if let Ok(body) = std::str::from_utf8(&bytes) {
    //     println!("body = {body:?}");
    // }

    let parsed = serde_json::from_slice::<T>(&bytes)?;

    Ok(parsed)
}

/// Helper functions to create a request
#[allow(dead_code)]
pub async fn request(method: Method, uri: &str) -> Response<Body> {
    let request = get_request(method, uri);
    let app = create_test_router().await;

    app.oneshot(request.await).await.unwrap()
}

/// Helper function to create a request with a body
#[allow(dead_code)]
pub async fn request_with_body<T: serde::Serialize>(
    method: Method,
    uri: &str,
    payload: &T,
) -> Response<Body> {
    let json_payload = serde_json::to_string(payload).expect("Failed to serialize payload");
    let request = get_request_with_body(method, uri, &json_payload);
    let app = create_test_router().await;

    app.oneshot(request.await).await.unwrap()
}

/// Helper function to create a request with authentication
#[allow(dead_code)]
pub async fn request_with_auth(method: Method, uri: &str) -> Response<Body> {
    let token = get_authentication_token().await;
    let request = get_request_with_auth(method, uri, &token);
    let app = create_test_router().await;

    app.oneshot(request.await).await.unwrap()
}

/// Helper function to create a request with authentication and a body
#[allow(dead_code)]
pub async fn request_with_auth_and_body<T: serde::Serialize>(
    method: Method,
    uri: &str,
    payload: &T,
) -> Response<Body> {
    let json_payload = serde_json::to_string(payload).expect("Failed to serialize payload");
    let token = get_authentication_token().await;
    let request = get_request_with_auth_and_body(method, uri, &token, &json_payload);
    let app = create_test_router().await;

    app.oneshot(request.await).await.unwrap()
}

/// Helper function to create a request with authentication and multipart data
#[allow(dead_code)]
pub async fn request_with_auth_and_multipart(
    method: Method,
    uri: &str,
    payload: Vec<u8>,
) -> Response<Body> {
    let token = get_authentication_token().await;
    let request = get_request_with_auth_and_multipart(method, uri, &token, payload);
    let app = create_test_router().await;

    app.oneshot(request.await).await.unwrap()
}

/// internal helper functions to create requests
async fn get_request(method: Method, uri: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri.to_string())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .body(axum::body::Body::empty())
        .unwrap()
}

/// internal helper function to create a request with a body
async fn get_request_with_body(method: Method, uri: &str, payload: &str) -> Request<Body> {
    let request: Request<Body> = Request::builder()
        .method(method)
        .uri(uri.to_string())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .body(axum::body::Body::from(payload.to_string()))
        .unwrap();

    request
}

/// internal helper function to create a request with authorization
async fn get_request_with_auth(method: Method, uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri.to_string())
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, token)
        .header(ACCEPT, "application/json")
        .body(axum::body::Body::empty())
        .unwrap()
}

async fn get_request_with_auth_and_body(
    method: Method,
    uri: &str,
    token: &str,
    payload: &str,
) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri.to_string())
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, token)
        .header(ACCEPT, "application/json")
        .body(axum::body::Body::from(payload.to_string()))
        .unwrap()
}

async fn get_request_with_auth_and_multipart(
    method: Method,
    uri: &str,
    token: &str,
    payload: Vec<u8>,
) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri.to_string())
        .header(CONTENT_TYPE, "multipart/form-data; boundary=----XYZ")
        .header(AUTHORIZATION, token)
        .header(ACCEPT, "application/json")
        .body(Body::from(payload))
        .unwrap()
}
