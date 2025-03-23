use std::env;
use axum::{Router, routing::get, Extension};
use std::net::SocketAddr;
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::signal;

mod handlers;
mod db;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    tracing::info!("{}", env::var("DATABASE_URL").unwrap_or_else(|err| format!("Error: {}", err)));
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "portfolio_api=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database connection
    let pool = db::connection::connect().await?;

    // Build application with routes
    let app = Router::new()
        .route("/projects", get(handlers::projects::get_projects))
        .layer(Extension(pool.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 Server running on {}", addr);

    // Start server with graceful shutdown
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.as_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received, starting graceful shutdown");
}
