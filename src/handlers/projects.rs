use axum::{Json, extract::State, http::StatusCode};
use crate::db::projects_db;
use axum::response::IntoResponse;
use sqlx::PgPool;
use tracing::error;
use crate::models::project::Project;

/// Get all projects
/// 
/// Returns a list of all projects in the portfolio
#[utoipa::path(
    get,
    path = "/projects",
    responses(
        (status = 200, description = "List of projects retrieved successfully", body = Vec<Project>),
        (status = 500, description = "Internal server error")
    ),
    tag = "projects"
)]
pub async fn get_projects(State(pool): State<PgPool>) -> impl IntoResponse {
    match projects_db::fetch_projects(&pool).await {
        Ok(projects) => (StatusCode::OK, Json(projects)).into_response(),
        Err(e) => {
            error!("Failed to get projects: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch projects").into_response()
        }
    }
}

/// Get a single project by ID
/// 
/// Returns a single project if found, or 404 if not found
#[utoipa::path(
    get,
    path = "/projects/{project_id}",
    responses(
        (status = 200, description = "Project retrieved successfully", body = Project),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("project_id" = i32, Path, description = "ID of the project to retrieve")
    ),
    tag = "projects"
)]
pub async fn get_project_by_id(
    State(pool): State<PgPool>,
    axum::extract::Path(project_id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match projects_db::fetch_project_by_id(&pool, project_id).await {
        Ok(Some(project)) => (StatusCode::OK, Json(project)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Project not found").into_response(),
        Err(e) => {
            error!("Failed to get project: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch project").into_response()
        }
    }
}

/// Get all projects for a specific job
/// 
/// Returns a list of all projects associated with the specified job
#[utoipa::path(
    get,
    path = "/projects/job/{job_id}",
    responses(
        (status = 200, description = "List of projects retrieved successfully", body = Vec<Project>),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("job_id" = i32, Path, description = "ID of the job to fetch projects for")
    ),
    tag = "projects"
)]
pub async fn get_projects_by_job(
    State(pool): State<PgPool>,
    axum::extract::Path(job_id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match projects_db::fetch_projects_by_job(&pool, job_id).await {
        Ok(projects) => (StatusCode::OK, Json(projects)).into_response(),
        Err(e) => {
            error!("Failed to get projects for job {}: {:?}", job_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch projects").into_response()
        }
    }
}
