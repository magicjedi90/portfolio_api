use crate::db::skills_db;
use crate::models::skill::Skill;
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;
use tracing::error;

/// Get all skills
///
/// Returns a list of all skills in the portfolio
#[utoipa::path(
    get,
    path = "/skills",
    responses(
        (status = 200, description = "List of skills retrieved successfully", body = Vec<Skill>),
        (status = 500, description = "Internal server error")
    ),
    tag = "skills"
)]
pub async fn get_skills(State(pool): State<PgPool>) -> impl IntoResponse {
    match skills_db::fetch_skills(&pool).await {
        Ok(skills) => (StatusCode::OK, Json(skills)).into_response(),
        Err(e) => {
            error!("Failed to get skills: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch skills").into_response()
        }
    }
}

/// Get a single skill by ID
///
/// Returns a single skill if found, or 404 if not found
#[utoipa::path(
    get,
    path = "/skills/{skill_id}",
    responses(
        (status = 200, description = "Skill retrieved successfully", body = Skill),
        (status = 404, description = "Skill not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("skill_id" = i32, Path, description = "ID of the skill to retrieve")
    ),
    tag = "skills"
)]
pub async fn get_skill_by_id(
    State(pool): State<PgPool>,
    axum::extract::Path(skill_id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match skills_db::fetch_skill_by_id(&pool, skill_id).await {
        Ok(Some(skill)) => (StatusCode::OK, Json(skill)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Skill not found").into_response(),
        Err(e) => {
            error!("Failed to get skill: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch skill").into_response()
        }
    }
}


