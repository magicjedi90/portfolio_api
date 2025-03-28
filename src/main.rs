use axum::{
    routing::get,
    Json,
};
use std::net::SocketAddr;
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::signal;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

mod handlers;
mod db;
mod models;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::projects::get_projects,
    ),
    components(
        schemas(
            models::project::Project,
            models::skill::Skill,
            crate::db::proficiency_enum::Proficiency
        )
    ),
    tags(
        (name = "projects", description = "Project management endpoints")
    ),
    info(
        title = "Portfolio API",
        version = "0.1.0",
        description = "API for managing portfolio projects and skills"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "portfolio_api=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database connection
    let pool = match db::connection::connect().await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Build our application with routes using OpenApiRouter
    let (router, _) = OpenApiRouter::new()
        .routes(routes!(handlers::projects::get_projects))
        .split_for_parts();

    let app = router
        .route("/api-docs/openapi.json", get(|| async { Json(ApiDoc::openapi()) }))
        .with_state(pool);

    // TODO make this configurable but use for default local run
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server running on {}", addr);
    tracing::info!("OpenAPI documentation available at http://{}/api-docs/openapi.json", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
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
