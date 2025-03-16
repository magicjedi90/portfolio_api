use serde::{Serialize, Deserialize};
use crate::models::technology::Technology;

/// Represents a portfolio project with its details and metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    /// Unique identifier for the project
    pub id: i32,
    /// Title of the project
    pub title: String,
    /// Detailed description of the project
    pub description: String,
    /// List of technologies used in the project
    pub tech_stack: Vec<Technology>,
    /// Optional GitHub repository URL for the project
    pub github_url: Option<String>,
}
