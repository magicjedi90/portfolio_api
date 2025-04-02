use serde::{Deserialize, Serialize};
use crate::db::proficiency_enum::Proficiency;
use utoipa::ToSchema;

/// Represents a technology/tool used in projects
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Skill {
    /// Unique identifier for the technology
    pub id: i32,
    /// Name of the technology
    pub name: String,
    /// Description of the technology
    pub description: String,
    /// Official website URL for the technology
    pub official_site_url: String,
    /// Proficiency level in the technology
    pub proficiency: Proficiency,
    pub parent_id: Option<i32>,
}
