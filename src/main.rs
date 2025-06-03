use clean_axum_demo::{app::create_router, common};
use common::{
    bootstrap::{build_app_state, shutdown_signal},
    config::{setup_database, Config},
};
use tracing::info;

#[cfg(not(feature = "opentelemetry"))]
use common::bootstrap::setup_tracing;

#[cfg(feature = "opentelemetry")]
use common::opentelemetry::{setup_tracing_opentelemetry, shutdown_opentelemetry};

/// Main entry point for the application.
/// It sets up the database connection, initializes the server, and starts listening for requests.
/// It also sets up the Swagger UI for API documentation.
///
/// # Errors
/// Returns an error if the database connection fails or if the server fails to start.
/// # Panics
/// Panics if the environment variables are not set correctly or if the server fails to start.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(not(feature = "opentelemetry"))]
    setup_tracing();

    #[cfg(feature = "opentelemetry")]
    let opentelemetry_tracer_provider = {
        let provider = setup_tracing_opentelemetry();
        // Startup span to ensure at least one span is generated and exported
        let span = tracing::info_span!("startup");
        let _enter = span.enter();
        provider
    };

    let config = Config::from_env()?;
    let pool = setup_database(&config).await?;
    let state = build_app_state(pool, config.clone());
    let app = create_router(state);

    let addr = format!("{}:{}", config.service_host, config.service_port);

    info!("Server running at {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    #[cfg(feature = "opentelemetry")]
    shutdown_opentelemetry(opentelemetry_tracer_provider)?;

    Ok(())
}
