use crate::api_docs::ApiDoc;
use crate::handlers::projects::{get_project_by_id, get_projects};
use crate::handlers::jobs::{get_job_by_id, get_jobs};
use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

/// Creates and configures all API routes
pub fn create_router(pool: PgPool) -> Router {
    // Create the base router
    let app = Router::new();

    // Create nested routers for each resource type
    let projects_router = Router::new()
        .route("/", get(get_projects))
        .route("/{project_id}", get(get_project_by_id));

    let jobs_router = Router::new()
        .route("/", get(get_jobs))
        .route("/{job_id}", get(get_job_by_id));

    let config = Config::new(["/api-docs/openapi.json"]);
    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .config(config)
        .url("/api-docs/openapi.json", ApiDoc::openapi());

    // Configure CORS
    // TODO tighten up security before prod deployment - make this biz configurable
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Nest the routers under their respective paths
    app.nest("/projects", projects_router)
        .nest("/jobs", jobs_router)
        .merge(swagger_ui)
        .layer(cors)
        .with_state(pool)
}
