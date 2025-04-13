use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::projects::get_projects,
        crate::handlers::projects::get_project_by_id,
        crate::handlers::projects::get_projects_by_job,
        crate::handlers::projects::get_projects_by_skill,
        crate::handlers::jobs::get_jobs,
        crate::handlers::jobs::get_job_by_id,
        crate::handlers::skills::get_skills,
        crate::handlers::skills::get_skill_by_id,
    ),
    components(
        schemas(
            crate::models::project::Project,
            crate::models::skill::Skill,
            crate::models::job::Job,
            crate::db::proficiency_enum::Proficiency
        )
    ),
    tags(
        (name = "projects", description = "Project management endpoints"),
        (name = "jobs", description = "Job history endpoints"),
        (name = "skills", description = "Skills management endpoints")
    ),
    info(
        title = "Portfolio API",
        version = "0.1.0",
        description = "API for managing portfolio projects, skills, and job history"
    )
)]
pub struct ApiDoc;
