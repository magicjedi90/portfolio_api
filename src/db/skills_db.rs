use crate::models::skill::Skill;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::Row;
use tracing::error;

const SKILL_QUERY: &str = r#"
    SELECT 
        id,
        name,
        description,
        official_site_url,
        proficiency,
        parent_id
    FROM skills
"#;

fn map_row_to_skill(row: PgRow) -> Skill {
    let proficiency = match row.try_get::<crate::db::proficiency_enum::Proficiency, _>("proficiency") {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to get proficiency from database row: {:?}", e);
            error!("Raw proficiency value: {:?}", row.try_get::<String, _>("proficiency"));
            crate::db::proficiency_enum::Proficiency::Beginner
        }
    };

    Skill {
        id: row.try_get("id").unwrap_or_default(),
        name: row.try_get("name").unwrap_or_default(),
        description: row.try_get("description").unwrap_or_default(),
        official_site_url: row.try_get("official_site_url").unwrap_or_default(),
        proficiency,
        parent_id: row.try_get("parent_id").unwrap_or_default(),
    }
}

/// Fetches all skills from the database, ordered by id.
///
/// # Arguments
///
/// * `pool` - The database connection pool
///
/// # Returns
///
/// * `Result<Vec<Skill>, sqlx::Error>` - A vector of skills if successful, or a database error
pub async fn fetch_skills(pool: &PgPool) -> Result<Vec<Skill>, sqlx::Error> {
    let rows = sqlx::query(format!("{} ORDER BY id ASC", SKILL_QUERY).as_str())
        .map(map_row_to_skill)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

/// Fetches a single skill by ID from the database.
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `skill_id` - The ID of the skill to fetch
///
/// # Returns
///
/// * `Result<Option<Skill>, sqlx::Error>` - The skill if found, None if not found, or a database error
pub async fn fetch_skill_by_id(pool: &PgPool, skill_id: i32) -> Result<Option<Skill>, sqlx::Error> {
    let row = sqlx::query(format!("{} WHERE id = $1", SKILL_QUERY).as_str())
        .bind(skill_id)
        .map(map_row_to_skill)
        .fetch_optional(pool)
        .await?;

    Ok(row)
} 