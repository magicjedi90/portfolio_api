use crate::models::project::Project;
use sqlx::PgPool;

pub async fn fetch_projects(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as!(Project, "SELECT * FROM projects")
        .fetch_all(pool)
        .await
}
