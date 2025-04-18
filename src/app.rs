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
    device::routes::device_routes,
    file::routes::file_routes,
    shared::{
        app_state::AppState,
        config::Config,
        error::{handle_error, AppError},
        jwt,
    },
    user::routes::user_routes,
};

use crate::auth::handlers::UserAuthApiDoc;
use crate::device::handlers::DeviceApiDoc;
use crate::file::handlers::FileApiDoc;
use crate::user::handlers::UserApiDoc;
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

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(1800))
        .layer(cors)
        .layer(middleware::from_fn(print_request_response));

    // setup public routes
    let public_routes = Router::new().nest("/auth", user_auth_routes());

    // setup protected routes
    let protected_routes = Router::new()
        .nest("/users", user_routes())
        .nest("/devices", device_routes())
        .nest("/files", file_routes())
        .route_layer(middleware::from_fn(jwt::jwt_auth));

    // setup assets routes
    let assets_routes = Router::new().nest_service(
        config.assets_public_url.as_str(),
        ServeDir::new(config.assets_public_path),
    );

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

pub async fn fallback() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::NOT_FOUND, "Not Found"))
}

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

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
