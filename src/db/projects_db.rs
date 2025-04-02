use crate::models::project::Project;
use sqlx::PgPool;
use crate::models::skill::Skill;
use sqlx::postgres::PgRow;
use sqlx::Row;
use serde_json::Value;

const PROJECT_SKILLS_QUERY: &str = r#"
    WITH project_skills AS (
        SELECT 
            p.id as project_id,
            COALESCE(
                jsonb_agg(
                    jsonb_build_object(
                        'id', s.id,
                        'name', s.name,
                        'description', s.description,
                        'official_site_url', s.official_site_url,
                        'proficiency', s.proficiency
                    )
                ) FILTER (WHERE s.id IS NOT NULL),
                '[]'::jsonb
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
        COALESCE(ps.skills, '[]'::jsonb) as skills
    FROM projects p
    LEFT JOIN project_skills ps ON p.id = ps.project_id
"#;

fn map_row_to_project(row: PgRow) -> Project {
    let skills_json: Value = row.try_get("skills").unwrap_or(Value::Array(vec![]));
    let skills: Vec<Skill> = serde_json::from_value(skills_json).unwrap_or_default();
        
    Project {
        id: row.try_get("id").unwrap_or_default(),
        name: row.try_get("name").unwrap_or_default(),
        description: row.try_get("description").unwrap_or_default(),
        github_url: row.try_get("github_url").unwrap_or_default(),
        job_id: row.try_get("job_id").unwrap_or_default(),
        skills,
    }
}

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
    let rows = sqlx::query(PROJECT_SKILLS_QUERY)
        .map(map_row_to_project)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

/// Fetches a single project by ID from the database.
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `project_id` - The ID of the project to fetch
///
/// # Returns
///
/// * `Result<Option<Project>, sqlx::Error>` - The project if found, None if not found, or a database error
pub async fn fetch_project_by_id(pool: &PgPool, project_id: i32) -> Result<Option<Project>, sqlx::Error> {
    let row = sqlx::query(format!("{} WHERE p.id = $1", PROJECT_SKILLS_QUERY).as_str())
        .bind(project_id)
        .map(map_row_to_project)
        .fetch_optional(pool)
        .await?;

    Ok(row)
}
