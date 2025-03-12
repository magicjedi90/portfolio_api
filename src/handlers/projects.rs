use axum::{Json, extract::State};
use crate::db::projects_db;
use crate::models::project::Project;
use axum::response::IntoResponse;
use sqlx::PgPool;

pub async fn get_projects(State(pool): State<PgPool>) -> impl IntoResponse {
    let projects = projects_db::fetch_projects(&pool).await.unwrap();
    Json(projects)
}
