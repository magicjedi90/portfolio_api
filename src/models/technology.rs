use serde::{Deserialize, Serialize};

/// Represents a technology/tool used in projects
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Technology {
    /// Unique identifier for the technology
    pub id: i32,
    /// Name of the technology
    pub name: String,
}
