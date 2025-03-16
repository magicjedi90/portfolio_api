use crate::models::project::{Project};
use sqlx::PgPool;
use tracing::error;

/// Fetches all projects from the database, ordered by id.
///
/// # Arguments
///
/// * `pool` - The database connection pool
///
/// # Returns
///
/// * `Result<Vec<Project>, sqlx::Error>` - A vector of projects if successful, or a database error
pub async fn fetch_projects(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
    // First, fetch all projects
    let mut projects = sqlx::query_as!(
        Project,
        r#"
        SELECT 
            p.id,
            p.title,
            p.description,
            p.github_url,
            ARRAY[]::record[]::technology[] as "tech_stack!"
        FROM projects p
        ORDER BY p.id ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    // Then, for each project, fetch its technologies
    for project in &mut projects {
        let technologies = sqlx::query_as!(
            Technology,
            r#"
            SELECT t.id, t.name
            FROM technologies t
            INNER JOIN projects_tech pt ON pt.technology_id = t.id
            WHERE pt.project_id = $1
            ORDER BY t.name ASC
            "#,
            project.id
        )
        .fetch_all(pool)
        .await?;

        project.tech_stack = technologies;
    }

    Ok(projects)
}
