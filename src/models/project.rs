use serde::{Serialize, Deserialize};
use crate::models::skill::Skill;

/// Represents a portfolio project with its details and metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    /// Unique identifier for the project
    pub id: i32,
    /// Title of the project
    pub name: String,
    /// Detailed description of the project
    pub description: String,
    /// Optional GitHub repository URL for the project
    pub github_url: Option<String>,
    pub job_id: Option<i32>,
    /// List of technologies used in the project
    pub skills: Vec<Skill>,
}
