use axum::{
    body::{Body, Bytes},
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use http_body_util::BodyExt;

use clean_axum_demo::{
    auth::controller::user_auth_handlers::login_user,
    shared::{
        app_state::AppState,
        config::Config,
        jwt::{AuthBody, AuthPayload},
    },
};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};

use std::str::FromStr;

pub async fn setup_test_db_state() -> Result<AppState, Box<dyn std::error::Error>> {
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
        .max_connections(2)
        .min_connections(1)
        .connect_with(connect_options)
        .await?;

    // Optional: set timezone in session
    sqlx::query("SET time_zone = '+00:00'")
        .execute(&pool)
        .await
        .expect("Failed to set timezone");

    Ok(AppState::new(pool, config.clone()))
}

#[allow(dead_code)]
pub async fn print_response(body: Body) {
    buffer_and_print("response", body).await;
}

async fn buffer_and_print<B>(direction: &str, body: B)
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            tracing::error!("failed to read {direction} body: {err}");
            return;
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        println!("{direction} body = {body:?}");
    }
}

#[allow(dead_code)]
pub async fn get_user_token() -> String {
    let state = setup_test_db_state().await;

    let payload = AuthPayload {
        client_id: "apitest01".to_string(),
        client_secret: "test_password".to_string(),
    };

    let response = match login_user(State(state.unwrap()), Json(payload)).await {
        Ok(resp) => resp.into_response(),
        Err(err) => {
            println!("Failed to login user: {:?}", err);
            return "".to_string();
        }
    };

    let (parts, body) = response.into_parts();

    if parts.status != StatusCode::OK {
        println!("Failed to get token, status: {:?}", parts.status);
        return "".to_string();
    }

    let body_string = get_body_string(body).await;
    let auth_body: AuthBody = serde_json::from_str(&body_string).unwrap();

    format!("{} {}", auth_body.token_type, auth_body.access_token)
}

async fn get_body_string<B>(body: B) -> String
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            tracing::error!("failed to read body: {err}");
            return "".to_string();
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        return body.to_string();
    } else {
        return "".to_string();
    }
}
