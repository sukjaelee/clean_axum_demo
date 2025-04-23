use axum::{
    body::{Body, Bytes},
    error_handling::HandleErrorLayer,
    extract::Request,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    middleware::{self, Next},
    response::IntoResponse,
    Router,
};
use http_body_util::BodyExt;

use sqlx::MySqlPool;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;

use crate::{
    auth::routes::user_auth_routes,
    common::{
        app_state::AppState,
        config::Config,
        error::{handle_error, AppError},
        jwt,
    },
    device::routes::device_routes,
    file::routes::file_routes,
    user::routes::user_routes,
};

use crate::auth::routes::UserAuthApiDoc;
use crate::device::routes::DeviceApiDoc;
use crate::file::routes::FileApiDoc;
use crate::user::routes::UserApiDoc;
use utoipa_swagger_ui::SwaggerUi;

fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/docs")
        .url(
            "/api-docs/user_auth/openapi.json",
            UserAuthApiDoc::openapi(),
        )
        .url("/api-docs/user/openapi.json", UserApiDoc::openapi())
        .url("/api-docs/device/openapi.json", DeviceApiDoc::openapi())
        .url("/api-docs/file/openapi.json", FileApiDoc::openapi())
}

async fn health_check() -> &'static str {
    "OK\n"
}

pub fn create_router(pool: MySqlPool, config: Config) -> Router {
    // Create the shared state by calling the constructor in config.rs
    let state = AppState::new(pool, config.clone());

    // Create cors middleware
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    // setup handler for errors, cors, timeout, and logging
    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(1800))
        .layer(cors)
        .layer(middleware::from_fn(print_request_response));

    // setup public routes
    let public_routes = Router::new().nest("/auth", user_auth_routes());

    // setup protected routes
    let protected_routes = Router::new()
        .nest("/user", user_routes())
        .nest("/device", device_routes())
        .nest("/file", file_routes())
        .route_layer(middleware::from_fn(jwt::jwt_auth));

    // setup assets routes
    let assets_routes = Router::new().nest_service(
        config.assets_public_url.as_str(),
        ServeDir::new(config.assets_public_path),
    );

    // Create the main router
    // and merge all the routes
    // and add the middleware stack
    // and add the state
    Router::new()
        .route("/health", axum::routing::get(health_check))
        .merge(public_routes)
        .merge(protected_routes)
        .merge(create_swagger_ui())
        .merge(assets_routes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "request",
                        method = %req.method(),
                        uri = %req.uri()
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        tracing::info!(
                            "request completed: status = {status}, latency = {latency:?}",
                            status = response.status(),
                            latency = latency
                        );
                    },
                ),
        )
        .fallback(fallback)
        .layer(middleware_stack)
        .with_state(state)
}

/// Setup tracing for the application.
/// This function initializes the tracing subscriber with a default filter and formatting.
pub fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "debug,sqlx=debug,tower_http=info,axum::rejection=trace".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE),
        )
        .init();
}

/// Fallback handler for unmatched routes
/// This function returns a 404 Not Found response with a message.
pub async fn fallback() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::NOT_FOUND, "Not Found"))
}

/// Shutdown signal handler
/// This function listens for a shutdown signal (CTRL+C) and logs a message when received.
pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

/// Middleware to print request and response bodies
/// This function intercepts the request and response, buffers the body, and prints it to the log.
/// It is used for debugging purposes to inspect the request and response data.
async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    // Uncomment the following lines to print the response body
    // let (parts, body) = res.into_parts();
    // let bytes = buffer_and_print("response", body).await?;
    // let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

/// Buffer and print the request/response body
/// This function collects the body data into bytes and prints it to the log.
async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{direction} body = {body:?}");
    }

    Ok(bytes)
}
