use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub github_url: String,
}
