use axum::Router;
use sqlx::{PgPool, Error};
use dotenv::dotenv;

/// Helper function to establish a database connection for integration tests.
///
/// Loads the database URL from the `.env` file or environment variables.
/// Make sure the DATABASE_URL is set to a test database.
pub async fn get_test_db_pool() -> Result<PgPool, Error> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL (make sure this is a test database!)
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in your .env file or as an environment variable");

    // Create and return the database pool.
    PgPool::connect(&database_url).await
}

pub async fn setup_router_with_test_db() -> Router {
    // Use your test DB pool setup
    let pool = get_test_db_pool()
        .await
        .expect("Failed to get test DB pool");

    // Pass the test pool to the router
    portfolio_api::routes::create_router(pool)
}