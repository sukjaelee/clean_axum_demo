use sqlx::MySqlPool; // Import AppError from the appropriate module

use crate::auth::services::AuthService;
use crate::common::app_state::AppState;
use crate::common::config::Config;
use crate::device::services::DeviceService;
use crate::file::services::FileService;
use crate::user::services::UserService;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Constructs and wires all application services and returns a configured AppState.
pub fn build_app_state(pool: MySqlPool, config: Config) -> AppState {
    let auth_service = AuthService::create_service(pool.clone());
    let file_service = FileService::create_service(config.clone(), pool.clone());
    let user_service = UserService::create_service(pool.clone(), file_service.clone());
    let device_service = DeviceService::create_service(pool.clone());

    AppState::new(
        config,
        auth_service,
        user_service,
        device_service,
        file_service,
    )
}

/// Setup tracing for the application.
/// This function initializes the tracing subscriber with a default filter and formatting.
pub fn setup_tracing() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=info,tower_http=info,axum::rejection=trace".into()),
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

/// Shutdown signal handler
/// This function listens for a shutdown signal (CTRL+C) and logs a message when received.
pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}
