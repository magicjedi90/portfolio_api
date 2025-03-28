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
