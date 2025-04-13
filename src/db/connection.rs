use sqlx::{Error, PgPool, postgres::PgPoolOptions};
use tracing::info;

// Refactored function to accept explicit configuration parameters
pub async fn connect_with_config(database_url: &str, max_connections: u32) -> Result<PgPool, Error> {
    info!("Attempting to connect to database...");

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await?;

    info!("Successfully connected to database!");

    Ok(pool)
}

// Wrapper function to preserve the original behavior (optional)
pub async fn connect() -> Result<PgPool, Error> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| Error::Configuration("DATABASE_URL environment variable is not set".into()))?;

    let max_connections: u32 = std::env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .unwrap_or(5);

    connect_with_config(&database_url, max_connections).await
}