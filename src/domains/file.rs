mod api {
    mod handlers;
    pub mod routes;
}

mod domain {
    pub mod model;
    pub mod repository;
    pub mod service;
}

pub mod dto {
    pub mod file_dto;
}

mod infra {
    mod impl_repository;
    pub mod impl_service;
}

// Re-export commonly used items for convenience
pub use api::routes::{file_routes, FileApiDoc};
pub use domain::service::FileServiceTrait;
pub use dto::file_dto::FileDto;
pub use infra::impl_service::FileService;
