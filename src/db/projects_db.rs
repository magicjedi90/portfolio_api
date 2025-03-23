use crate::models::project::Project;
use sqlx::PgPool;
use crate::models::skill::Skill;
use serde_json::Value;

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
    #[derive(sqlx::FromRow)]
    struct ProjectRow {
        id: i32,
        name: String,
        description: String,
        github_url: Option<String>,
        job_id: Option<i32>,
        skills: Value,
    }

    let rows = sqlx::query_as!(
        ProjectRow,
        r#"
        WITH project_skills AS (
            SELECT 
                p.id as project_id,
                COALESCE(
                    json_agg(
                        json_build_object(
                            'id', s.id,
                            'name', s.name,
                            'description', s.description,
                            'official_site_url', s.official_site_url,
                            'proficiency', s.proficiency
                        )
                    ) FILTER (WHERE s.id IS NOT NULL),
                    '[]'::json
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

    let projects = rows.into_iter().map(|row| {
        let skills: Vec<Skill> = serde_json::from_value(row.skills).unwrap_or_default();
        Project {
            id: row.id,
            name: row.name,
            description: row.description,
            github_url: row.github_url,
            job_id: row.job_id,
            skills,
        }
    }).collect();

    Ok(projects)
}
