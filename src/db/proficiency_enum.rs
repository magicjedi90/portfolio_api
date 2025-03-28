use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

/// Represents the proficiency level in a technology
#[derive(Debug, Serialize, Deserialize, Clone, Type, ToSchema)]
#[sqlx(type_name = "proficiency")]
pub enum Proficiency {
    /// Basic understanding and ability to use the technology
    Beginner,
    /// Good understanding and practical experience with the technology
    Intermediate,
    /// Deep understanding and extensive experience with the technology
    Advanced,
    /// Mastery of the technology with ability to teach and innovate
    Expert
}