use crate::models::project::{Project};
use sqlx::PgPool;
use tracing::error;
use crate::models::skill::Skill;

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
            p.name,
            p.description,
            p.github_url,
            p.job_id,
            ARRAY[]::record[]::skill[] as "tech_stack!"

        FROM projects p
        ORDER BY p.id ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    // Then, for each project, fetch its skills
    for project in &mut projects {
        let skills = sqlx::query_as!(
            Skill,
            r#"
            SELECT t.id, t.name, t.description, t.official_site_url, t.proficiency
            FROM skills t
            INNER JOIN projects_skills pt ON pt.skill_id = t.id
            WHERE pt.project_id = $1
            ORDER BY t.name ASC
            "#,
            project.id
        )
        .fetch_all(pool)
        .await?;

        project.skills = skills;
    }

    Ok(projects)
}
