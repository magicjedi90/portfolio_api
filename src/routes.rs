use axum::{Router, routing::get};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Config};
use crate::api_docs::ApiDoc;
use crate::handlers::projects::{get_projects, get_project_by_id};

/// Creates and configures all API routes
pub fn create_router(pool: PgPool) -> Router {
    // Create the base router
    let app = Router::new();

    // Create nested routers for each resource type
    let projects_router = Router::new()
        .route("/", get(get_projects))
        .route("/{project_id}", get(get_project_by_id));

    let config = Config::new(["/api-docs/openapi.json"]);
    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .config(config)
        .url("/api-docs/openapi.json", ApiDoc::openapi());

    // Nest the routers under their respective paths
    app.nest("/projects", projects_router)
        .merge(swagger_ui)
        .with_state(pool)
} 