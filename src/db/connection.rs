use sqlx::{Error, PgPool, postgres::PgPoolOptions};
use std::env;
use tracing::info;

pub async fn connect() -> Result<PgPool, Error> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| Error::Configuration("DATABASE_URL environment variable is not set".into()))?;

    info!("Attempting to connect to database...");

    let max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .unwrap_or(5);

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await?;

    info!("Successfully connected to database!");

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_database_connection() {
        dotenv().ok();
        match connect().await {
            Ok(_) => println!("Successfully connected to database!"),
            Err(e) => panic!("Failed to connect to database: {}", e),
        }
    }
}
