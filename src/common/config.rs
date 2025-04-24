use serde::Deserialize;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySqlPool,
};
use std::{env, str::FromStr};

/// Config is a struct that holds the configuration for the application.
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub database_charset: String,
    pub database_max_connections: u32,
    pub database_min_connections: u32,
    pub database_time_zone: String,

    pub service_host: String,
    pub service_port: String,

    pub assets_public_path: String,
    pub assets_public_url: String,

    pub assets_private_path: String,
    pub assets_private_url: String,

    pub asset_allowed_extensions: String,
    pub asset_max_size: usize,
}

/// from_env reads the environment variables and returns a Config struct.
/// It uses the dotenv crate to load environment variables from a .env file if it exists.
/// It returns a Result with the Config struct or an error if any of the environment variables are missing.
impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            database_charset: env::var("DATABASE_CHARSET")
                .unwrap_or_else(|_| "utf8mb4".to_string()),
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .map(|s| s.parse::<u32>().unwrap_or(5))
                .unwrap_or(5),
            database_min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .map(|s| s.parse::<u32>().unwrap_or(1))
                .unwrap_or(1),
            database_time_zone: env::var("DATABASE_TIME_ZONE")
                .unwrap_or_else(|_| "+00:00".to_string()),

            service_host: env::var("SERVICE_HOST")?,
            service_port: env::var("SERVICE_PORT")?,

            assets_public_path: env::var("ASSETS_PUBLIC_PATH")?,
            assets_public_url: env::var("ASSETS_PUBLIC_URL")?,

            assets_private_path: env::var("ASSETS_PRIVATE_PATH")?,
            assets_private_url: env::var("ASSETS_PRIVATE_URL")?,

            asset_allowed_extensions: env::var("ASSET_ALLOWED_EXTENSIONS")
                .unwrap_or_else(|_| "jpg,jpeg,png,gif,webp".to_string()),
            asset_max_size: env::var("ASSET_MAX_SIZE")
                .map(|s| s.parse::<usize>().unwrap_or(50 * 1024 * 1024))?, // Default to 50MB
        })
    }
}

/// setup_database initializes the database connection pool.
pub async fn setup_database(config: &Config) -> Result<MySqlPool, sqlx::Error> {
    // Create connection options
    let connect_options = MySqlConnectOptions::from_str(&config.database_url)
        .map_err(|e| {
            tracing::error!("Failed to parse database URL: {}", e);
            e
        })?
        .charset(&config.database_charset)
        .clone();

    // Avoid using problematic timezone settings unless absolutely required
    // If you must set timezone, do it in SQL after connect

    let pool = MySqlPoolOptions::new()
        .max_connections(config.database_max_connections)
        .min_connections(config.database_min_connections)
        .connect_with(connect_options)
        .await?;

    // Optional: set timezone in session
    sqlx::query(&format!("SET time_zone = '{}'", config.database_time_zone))
        .execute(&pool)
        .await?;

    Ok(pool)
}
