use crate::db::proficiency_enum::Proficiency;
use crate::models::skill::Skill;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::{Error, Row};
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
    let proficiency = match row.try_get::<Proficiency, _>("proficiency") {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to get proficiency from database row: {:?}", e);
            error!(
                "Raw proficiency value: {:?}",
                row.try_get::<String, _>("proficiency")
            );
            Proficiency::Beginner
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

/// Implements the `SkillsRepository` trait for `PgPool` (used in production)

pub async fn fetch_skills(pool: &PgPool) -> Result<Vec<Skill>, Error> {
    let query = format!("{} ORDER BY id ASC", SKILL_QUERY);
    sqlx::query(&query)
        .map(map_row_to_skill)
        .fetch_all(pool)
        .await
}

pub async fn fetch_skill_by_id(pool: &PgPool, skill_id: i32) -> Result<Option<Skill>, Error> {
    let query = format!("{} WHERE id = $1", SKILL_QUERY);
    sqlx::query(&query)
        .bind(skill_id)
        .map(map_row_to_skill)
        .fetch_optional(pool)
        .await
}
