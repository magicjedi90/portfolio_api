use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::projects::get_projects,
        crate::handlers::projects::get_project_by_id,
    ),
    components(
        schemas(
            crate::models::project::Project,
            crate::models::skill::Skill,
            crate::db::proficiency_enum::Proficiency
        )
    ),
    tags(
        (name = "projects", description = "Project management endpoints")
    ),
    info(
        title = "Portfolio API",
        version = "0.1.0",
        description = "API for managing portfolio projects and skills"
    )
)]
pub struct ApiDoc; 