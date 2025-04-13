use crate::models::project::Project;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::{Error, Row};

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
                    ) ORDER BY s.name ASC
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

/// Maps a database row to a `Project` struct.
fn map_row_to_project(row: PgRow) -> Project {
    Project {
        id: row.try_get("id").unwrap_or_default(),
        name: row.try_get("name").unwrap_or_default(),
        description: row.try_get("description").unwrap_or_default(),
        github_url: row.try_get("github_url").unwrap_or_default(),
        job_id: row.try_get("job_id").unwrap_or_default(),
        skills: serde_json::from_value(
            row.try_get::<serde_json::Value, _>("skills")
                .unwrap_or_default(),
        )
        .unwrap_or_default(),
    }
}

pub async fn fetch_projects(pool: &PgPool) -> Result<Vec<Project>, Error> {
    let query = format!("{} ORDER BY id ASC", PROJECT_SKILLS_QUERY);
    sqlx::query(&query)
        .map(map_row_to_project)
        .fetch_all(pool)
        .await
}

pub async fn fetch_project_by_id(pool: &PgPool, project_id: i32) -> Result<Option<Project>, Error> {
    let query = format!("{} WHERE p.id = $1", PROJECT_SKILLS_QUERY);
    sqlx::query(&query)
        .bind(project_id)
        .map(map_row_to_project)
        .fetch_optional(pool)
        .await
}

pub async fn fetch_projects_by_job(pool: &PgPool, job_id: i32) -> Result<Vec<Project>, Error> {
    let query = format!(
        "{} WHERE p.job_id = $1 ORDER BY id ASC",
        PROJECT_SKILLS_QUERY
    );
    sqlx::query(&query)
        .bind(job_id)
        .map(map_row_to_project)
        .fetch_all(pool)
        .await
}

pub async fn fetch_projects_by_skill(pool: &PgPool, skill_id: i32) -> Result<Vec<Project>, Error> {
    // Filter projects by skill (join on projects_skills mapping table)
    let query = format!(
        "{} WHERE p.id IN (SELECT project_id FROM projects_skills WHERE skill_id = $1) ORDER BY id ASC",
        PROJECT_SKILLS_QUERY
    );
    sqlx::query(&query)
        .bind(skill_id)
        .map(map_row_to_project)
        .fetch_all(pool)
        .await
}
