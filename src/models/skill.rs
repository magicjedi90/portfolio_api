use serde::{Deserialize, Serialize};
use crate::db::proficiency_enum::Proficiency;

/// Represents a technology/tool used in projects
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Skill {
    /// Unique identifier for the technology
    pub id: i32,
    /// Name of the technology
    pub name: String,
    pub description: String,
    pub official_site_url: String,
    pub proficiency: Proficiency,
}
