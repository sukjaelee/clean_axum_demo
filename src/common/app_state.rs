use crate::auth::services::AuthService;
use crate::device::services::DeviceService;
use crate::file::services::FileService;
use crate::user::services::UserService;
use sqlx::MySqlPool;
use std::sync::Arc;

use super::config::Config;

/// AppState is a struct that holds the application state.
/// It contains a database connection pool, configuration, and various services.
/// This struct is used to share state across different parts of the application.
/// It is designed to be cloneable, allowing it to be passed around in a thread-safe manner.
#[derive(Clone)]
pub struct AppState {
    pub config: Config,

    pub auth_service: AuthService,
    pub user_service: UserService,
    pub device_service: DeviceService,
    pub file_service: FileService,
    // Add other services as needed
}

/// Creates a new instance of AppState.
/// This function initializes the AppState with a database connection pool and configuration.
/// It also initializes the various services used in the application.
impl AppState {
    pub fn new(pool: MySqlPool, config: Config) -> Self {
        let auth_service = AuthService::new(pool.clone());
        let file_service = FileService::new(config.clone(), pool.clone());
        let user_service = UserService::new(pool.clone(), Arc::new(file_service.clone()));
        let device_service = DeviceService::new(pool.clone());

        // Add other services as needed

        // Return a new instance of AppState with the initialized services
        Self {
            config,
            auth_service,
            user_service,
            device_service,
            file_service,
        }
    }
}
