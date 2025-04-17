use app::create_router;
// src/main.rs
use shared::config::{setup_database, Config};
use tracing::info;

mod app;
mod auth;
mod device;
mod file;
mod shared;
mod user;

/// Main entry point for the application.
/// It sets up the database connection, initializes the server, and starts listening for requests.
/// It also sets up the Swagger UI for API documentation.
///
/// # Errors
/// Returns an error if the database connection fails or if the server fails to start.
/// # Panics
/// Panics if the environment variables are not set correctly or if the server fails to start.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::setup_tracing();

    let config = Config::from_env()?;
    let pool = setup_database(&config).await?;
    let app = create_router(pool, config.clone());

    let addr = format!("{}:{}", config.service_host, config.service_port);

    info!("Starting server on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(app::shutdown_signal())
        .await?;

    info!("Server stopped");

    Ok(())
}
