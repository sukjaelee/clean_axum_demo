use crate::auth::repository::{
    user_auth_queries::UserAuthRepo, user_auth_repository::UserAuthRepository,
};
use crate::device::reposiroty::{device_queries::DeviceRepo, device_repository::DeviceRepository};
use crate::file::repository::{file_queries::FileRepo, file_repository::FileRepository};
use crate::user::repository::{user_queries::UserRepo, user_repository::UserRepository};
use sqlx::MySqlPool;
use std::sync::Arc;

use super::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub config: Config,
    pub device_repo: Arc<dyn DeviceRepository + Send + Sync>,
    pub user_repo: Arc<dyn UserRepository + Send + Sync>,
    pub file_repo: Arc<dyn FileRepository + Send + Sync>,
    pub user_auth_repo: Arc<dyn UserAuthRepository + Send + Sync>,
}

impl AppState {
    pub fn new(pool: MySqlPool, config: Config) -> Self {
        // define the repositories (note the bounds ensure these Arc types are Send + Sync)
        let device_repo: Arc<dyn DeviceRepository + Send + Sync> = Arc::new(DeviceRepo {});
        let user_repo: Arc<dyn UserRepository + Send + Sync> = Arc::new(UserRepo {});
        let file_repo: Arc<dyn FileRepository + Send + Sync> = Arc::new(FileRepo {});
        let user_auth_repo: Arc<dyn UserAuthRepository + Send + Sync> = Arc::new(UserAuthRepo {});
        // Create a new AppState with the database pool and repositories
        Self {
            pool,
            config,
            device_repo,
            user_repo,
            file_repo,
            user_auth_repo,
        }
    }
}
