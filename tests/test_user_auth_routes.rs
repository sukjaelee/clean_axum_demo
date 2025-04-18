use axum::extract::State;

use axum::{http::StatusCode, response::IntoResponse, Json};

use clean_axum_demo::{
    app::create_router,
    auth::{
        dto::AuthUser,
        handlers::{create_user_auth, login_user},
    },
    shared::{config::Config, error::AppError, jwt::AuthPayload},
};

mod test_helpers;

use test_helpers::{print_response, setup_test_db_state};
use tower::ServiceExt;

#[tokio::test]
async fn test_create_device() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let payload = AuthUser {
        user_id: "00000000-0000-0000-0000-000000000001".to_string(),
        password: "test_password".to_string(),
    };

    let response = create_user_auth(State(state.unwrap()), Json(payload))
        .await?
        .into_response();

    let (parts, body) = response.into_parts();
    print_response(body).await;
    assert_eq!(parts.status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_login_user() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let payload = AuthPayload {
        client_id: "apitest01".to_string(),
        client_secret: "test_password".to_string(),
    };

    let response = login_user(State(state.unwrap()), Json(payload))
        .await?
        .into_response();

    let (parts, body) = response.into_parts();
    print_response(body).await;
    assert_eq!(parts.status, StatusCode::OK);

    Ok(())
}

// test login_user with Request::builder()
#[tokio::test]
async fn test_login_user_with_request() -> Result<(), AppError> {
    let state = setup_test_db_state().await;

    let payload = AuthPayload {
        client_id: "apitest01".to_string(),
        client_secret: "test_password".to_string(),
    };

    let request = axum::http::Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(
            serde_json::to_string(&payload).unwrap(),
        ))
        .unwrap();

    // Pass this request into your app/router to extract Multipart in the handler.
    let config = Config::from_env().map_err(|_e| AppError::InternalError)?;

    let app = create_router(state.unwrap().pool, config.clone());
    let response = app.oneshot(request).await.unwrap();

    let (parts, body) = response.into_parts();
    print_response(body).await;
    assert_eq!(parts.status, StatusCode::OK);

    Ok(())
}
