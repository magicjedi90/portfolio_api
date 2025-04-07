use crate::api_docs::ApiDoc;
use crate::handlers::projects::{get_project_by_id, get_projects, get_projects_by_job, get_projects_by_skill};
use crate::handlers::jobs::{get_job_by_id, get_jobs};
use crate::handlers::skills::{get_skill_by_id, get_skills};
use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};
use axum::http::HeaderValue;

/// Creates and configures all API routes
pub fn create_router(pool: PgPool) -> Router {
    // Create the base router
    let app = Router::new();

    // Create nested routers for each resource type
    let projects_router = Router::new()
        .route("/", get(get_projects))
        .route("/{project_id}", get(get_project_by_id))
        .route("/job/{job_id}", get(get_projects_by_job))
        .route("/skill/{skill_id}", get(get_projects_by_skill));

    let jobs_router = Router::new()
        .route("/", get(get_jobs))
        .route("/{job_id}", get(get_job_by_id));

    let skills_router = Router::new()
        .route("/", get(get_skills))
        .route("/{skill_id}", get(get_skill_by_id));

    let config = Config::new(["/api-docs/openapi.json"]);
    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .config(config)
        .url("/api-docs/openapi.json", ApiDoc::openapi());

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("https://sindbadmcintosh.com"))
        .allow_methods(Any)
        .allow_headers(Any);

    // Nest the routers under their respective paths
    app.nest("/projects", projects_router)
        .nest("/jobs", jobs_router)
        .nest("/skills", skills_router)
        .merge(swagger_ui)
        .layer(cors)
        .with_state(pool)
}
