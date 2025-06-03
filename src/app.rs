use axum::{
    body::{Body, Bytes},
    error_handling::HandleErrorLayer,
    extract::{DefaultBodyLimit, Request},
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    middleware::{self, Next},
    response::IntoResponse,
    Router,
};
use http_body_util::BodyExt;

use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};

use utoipa::OpenApi;

use crate::{
    auth::routes::user_auth_routes,
    common::{
        app_state::AppState,
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

pub fn create_router(state: AppState) -> Router {
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
    // by default, Multipart limits the request body size to 2MB.
    // See DefaultBodyLimit for how to configure this limit.
    // https://docs.rs/axum/latest/axum/extract/struct.Multipart.html
    let protected_routes = Router::new()
        .nest("/user", user_routes())
        .nest("/device", device_routes())
        .nest("/file", file_routes())
        .layer(DefaultBodyLimit::max(state.config.asset_max_size))
        .route_layer(middleware::from_fn(jwt::jwt_auth));

    // setup assets routes
    let public_assets_routes = Router::new().nest_service(
        state.config.assets_public_url.as_str(),
        ServeDir::new(state.config.assets_public_path.clone()),
    );

    let private_assets_routes = Router::new()
        .nest_service(
            state.config.assets_private_url.as_str(),
            ServeDir::new(state.config.assets_private_path.clone()),
        )
        .route_layer(middleware::from_fn(jwt::jwt_auth));

    // Create the main router
    // and merge all the routes
    // and add the middleware stack
    // and add the state
    Router::new()
        .route("/health", axum::routing::get(health_check))
        .merge(public_routes)
        .merge(protected_routes)
        .merge(create_swagger_ui())
        .merge(public_assets_routes)
        .merge(private_assets_routes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
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

async fn health_check() -> &'static str {
    "OK\n"
}

/// Fallback handler for unmatched routes
/// This function returns a 404 Not Found response with a message.
pub async fn fallback() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::NOT_FOUND, "Not Found"))
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
