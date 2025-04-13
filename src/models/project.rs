use crate::models::skill::Skill;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents a portfolio project with its details and metadata
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Project {
    /// Unique identifier for the project
    pub id: i32,
    /// Title of the project
    pub name: String,
    /// Detailed description of the project
    pub description: String,
    /// Optional GitHub repository URL for the project
    pub github_url: Option<String>,
    /// Optional job ID associated with the project
    pub job_id: Option<i32>,
    /// List of technologies used in the project
    pub skills: Vec<Skill>,
}
