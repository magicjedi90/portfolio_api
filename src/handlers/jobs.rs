use axum::{Json, extract::State, http::StatusCode};
use crate::db::jobs_db;
use axum::response::IntoResponse;
use sqlx::PgPool;
use tracing::error;
use crate::models::job::Job;

/// Get all jobs
/// 
/// Returns a list of all jobs in the portfolio
#[utoipa::path(
    get,
    path = "/jobs",
    responses(
        (status = 200, description = "List of jobs retrieved successfully", body = Vec<Job>),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
pub async fn get_jobs(State(pool): State<PgPool>) -> impl IntoResponse {
    match jobs_db::fetch_jobs(&pool).await {
        Ok(jobs) => (StatusCode::OK, Json(jobs)).into_response(),
        Err(e) => {
            error!("Failed to get jobs: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch jobs").into_response()
        }
    }
}

/// Get a single job by ID
/// 
/// Returns a single job if found, or 404 if not found
#[utoipa::path(
    get,
    path = "/jobs/{job_id}",
    responses(
        (status = 200, description = "Job retrieved successfully", body = Job),
        (status = 404, description = "Job not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("job_id" = i32, Path, description = "ID of the job to retrieve")
    ),
    tag = "jobs"
)]
pub async fn get_job_by_id(
    State(pool): State<PgPool>,
    axum::extract::Path(job_id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match jobs_db::fetch_job_by_id(&pool, job_id).await {
        Ok(Some(job)) => (StatusCode::OK, Json(job)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Job not found").into_response(),
        Err(e) => {
            error!("Failed to get job: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch job").into_response()
        }
    }
} 