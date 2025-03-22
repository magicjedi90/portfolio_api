use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// Represents a technology/tool used in projects
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Job {
    pub id: i32,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_current_job: bool,
    pub company_name: String,
    pub company_website: String,
    pub description: String,
    pub roles: String,
    pub responsibilities: String,
}