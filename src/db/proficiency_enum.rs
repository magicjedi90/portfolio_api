use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[sqlx(type_name = "proficiency")]
pub enum Proficiency {
    Beginner,
    Intermediate,
    Advanced,
    Expert
}