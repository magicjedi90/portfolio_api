use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use utoipa::ToSchema;

/// Represents a job in the portfolio
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Job {
    pub id: i32,
    #[schema(example = "2023-01-01")]
    pub start_date: NaiveDate,
    #[schema(example = "2024-01-01", nullable = true)]
    pub end_date: Option<NaiveDate>,
    pub is_current_job: bool,
    pub company_name: String,
    pub company_website: String,
    pub description: String,
    pub roles: String,
    pub responsibilities: String,
}