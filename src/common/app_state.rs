use std::sync::Arc;

use crate::domains::{
    auth::AuthServiceTrait, device::DeviceServiceTrait, file::FileServiceTrait,
    user::UserServiceTrait,
};

use super::config::Config;

/// AppState is a struct that holds the application-wide shared state.
/// It is passed to request handlers via Axum's extension mechanism.
#[derive(Clone)]
pub struct AppState {
    /// Global application configuration.
    pub config: Config,
    /// Service handling authentication-related logic.
    pub auth_service: Arc<dyn AuthServiceTrait>,
    /// Service handling user-related logic.
    pub user_service: Arc<dyn UserServiceTrait>,
    /// Service handling device-related logic.
    pub device_service: Arc<dyn DeviceServiceTrait>,
    /// Service handling file-related logic.
    pub file_service: Arc<dyn FileServiceTrait>,
}

impl AppState {
    /// Creates a new instance of AppState with the provided dependencies.
    pub fn new(
        config: Config,
        auth_service: Arc<dyn AuthServiceTrait>,
        user_service: Arc<dyn UserServiceTrait>,
        device_service: Arc<dyn DeviceServiceTrait>,
        file_service: Arc<dyn FileServiceTrait>,
    ) -> Self {
        Self {
            config,
            auth_service,
            user_service,
            device_service,
            file_service,
        }
    }
}
