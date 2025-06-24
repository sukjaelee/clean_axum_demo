use super::handlers::*;
use crate::{
    common::app_state::AppState,
    domains::device::dto::device_dto::{CreateDeviceDto, DeviceDto, UpdateDeviceDto},
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_device_by_id,
        get_devices,
        create_device,
        update_device,
        update_many_devices,
        delete_device,
    ),
    components(schemas(DeviceDto, CreateDeviceDto, UpdateDeviceDto)),
    tags(
        (name = "Device", description = "Device management endpoints")
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&DeviceApiDoc)
)]
/// This struct is used to generate OpenAPI documentation for the device routes.
pub struct DeviceApiDoc;

impl utoipa::Modify for DeviceApiDoc {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("Input your `<your‑jwt>`"))
                    .build(),
            ),
        )
    }
}

/// This function creates a router for the device routes.
/// It defines the routes and their corresponding handlers.
pub fn device_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_devices))
        .route("/", post(create_device))
        .route("/{id}", get(get_device_by_id))
        .route("/{id}", put(update_device))
        .route("/{id}", delete(delete_device))
        .route("/batch/{user_id}", put(update_many_devices))
}
