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
    let mut projects = sqlx::query_as!(
        Project,
        r#"
        WITH project_skills AS (
            SELECT 
                p.id as project_id,
                json_agg(
                    json_build_object(
                        'id', s.id,
                        'name', s.name,
                        'description', s.description,
                        'official_site_url', s.official_site_url,
                        'proficiency', s.proficiency
                    )
                ) as skills
            FROM projects p
            LEFT JOIN projects_skills ps ON p.id = ps.project_id
            LEFT JOIN skills s ON ps.skill_id = s.id
            GROUP BY p.id
        )
        SELECT 
            p.id,
            p.name,
            p.description,
            p.github_url,
            p.job_id,
            COALESCE(ps.skills, '[]'::json) as skills
        FROM projects p
        LEFT JOIN project_skills ps ON p.id = ps.project_id
        ORDER BY p.id ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(projects)
}
