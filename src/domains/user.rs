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
    pub mod user_dto;
}

mod infra {
    mod impl_repository;
    pub mod impl_service;
}

// Re-export commonly used items for convenience
pub use api::routes::{user_routes, UserApiDoc};
pub use domain::service::UserServiceTrait;
pub use infra::impl_service::UserService;
