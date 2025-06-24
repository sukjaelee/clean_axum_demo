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
    pub mod device_dto;
}

mod infra {
    mod impl_repository;
    pub mod impl_service;
}

// Re-export commonly used items for convenience
pub use api::routes::{device_routes, DeviceApiDoc};
pub use domain::model::{DeviceOS, DeviceStatus};
pub use domain::service::DeviceServiceTrait;
pub use infra::impl_service::DeviceService;
