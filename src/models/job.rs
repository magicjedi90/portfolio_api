use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// Represents a technology/tool used in projects
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Job {
    pub id: i32,
    pub start_date: NaiveDate,
}