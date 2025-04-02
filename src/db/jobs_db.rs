use crate::models::job::Job;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::Row;

const JOB_QUERY: &str = r#"
    SELECT 
        id,
        start_date,
        end_date,
        is_current_job,
        company_name,
        company_website,
        description,
        roles,
        responsibilities
    FROM jobs
"#;

fn map_row_to_job(row: PgRow) -> Job {
    Job {
        id: row.try_get("id").unwrap_or_default(),
        start_date: row.try_get("start_date").unwrap_or_default(),
        end_date: row.try_get("end_date").unwrap_or_default(),
        is_current_job: row.try_get("is_current_job").unwrap_or_default(),
        company_name: row.try_get("company_name").unwrap_or_default(),
        company_website: row.try_get("company_website").unwrap_or_default(),
        description: row.try_get("description").unwrap_or_default(),
        roles: row.try_get("roles").unwrap_or_default(),
        responsibilities: row.try_get("responsibilities").unwrap_or_default(),
    }
}

/// Fetches all jobs from the database, ordered by id.
///
/// # Arguments
///
/// * `pool` - The database connection pool
///
/// # Returns
///
/// * `Result<Vec<Job>, sqlx::Error>` - A vector of jobs if successful, or a database error
pub async fn fetch_jobs(pool: &PgPool) -> Result<Vec<Job>, sqlx::Error> {
    let rows = sqlx::query(format!("{} ORDER BY id ASC", JOB_QUERY).as_str())
        .map(map_row_to_job)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

/// Fetches a single job by ID from the database.
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `job_id` - The ID of the job to fetch
///
/// # Returns
///
/// * `Result<Option<Job>, sqlx::Error>` - The job if found, None if not found, or a database error
pub async fn fetch_job_by_id(pool: &PgPool, job_id: i32) -> Result<Option<Job>, sqlx::Error> {
    let row = sqlx::query(format!("{} WHERE id = $1", JOB_QUERY).as_str())
        .bind(job_id)
        .map(map_row_to_job)
        .fetch_optional(pool)
        .await?;

    Ok(row)
} 