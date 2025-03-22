use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "proficiency")] // This should match the name of the enum type in the database
#[sqlx(rename_all = "lowercase")] // Ensures mapping matches case (PostgreSQL enums are usually lowercase)
pub enum Proficency {
    GettingStarted,
    Proficient,
    Expert,
}