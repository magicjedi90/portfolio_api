
use portfolio_api::db::connection::{connect, connect_with_config};
use dotenv::dotenv;

#[tokio::test]
async fn test_database_connection() {
    // Load .env file for defaults
    dotenv().ok();

    let result = connect().await;
    assert!(result.is_ok(), "Database connection failed: {:?}", result.err());
}

#[tokio::test]
async fn test_database_connection_failure_missing_url() {
    // Provide test-specific invalid configurations
    let database_url = ""; // Invalid URL
    let max_connections = 5;

    let result = connect_with_config(database_url, max_connections).await;
    assert!(result.is_err(), "Expected error when database_url is invalid");
}
