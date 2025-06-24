use super::handlers::*;
use crate::{common::app_state::AppState, domains::file::dto::file_dto::UploadedFileDto};
use axum::{
    routing::{delete, get},
    Router,
};

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        serve_protected_file,
        delete_file,
    ),
    components(schemas(UploadedFileDto)),
    tags(
        (name = "Files", description = "File management endpoints")
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&FileApiDoc)
)]
/// FileApiDoc is used to generate OpenAPI documentation for the file API.
pub struct FileApiDoc;

impl utoipa::Modify for FileApiDoc {
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

pub fn file_routes() -> Router<AppState> {
    Router::new()
        .route("/{file_id}", get(serve_protected_file))
        .route("/{file_id}", delete(delete_file))
}
