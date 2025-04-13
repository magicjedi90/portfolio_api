use hyper::Request;
use tower::ServiceExt; // For testing axum routes
use serde_json;
use portfolio_api::models::project::Project;
use crate::integration::test_utils::setup_router_with_test_db;

#[tokio::test]
async fn test_fetch_projects_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP GET request to `/projects`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/projects")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Ensure response status is OK
    assert_eq!(response.status(), 200);

    // Parse and verify the response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let projects: Vec<Project> = serde_json::from_slice(&body).expect("Failed to parse response body");

    assert!(!projects.is_empty(), "Projects should not be empty");
    println!("Fetched projects: {:?}", projects);
}

#[tokio::test]
async fn test_fetch_project_by_id_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP GET request to `/projects/10`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/projects/10")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Make sure response status is OK
    assert_eq!(response.status(), 200);

    // Parse and verify the response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let project: Project = serde_json::from_slice(&body).expect("Failed to parse response body");

    assert_eq!(project.id, 10, "Expected project ID to be 10");
    println!("Fetched project: {:?}", project);
}

#[tokio::test]
async fn test_fetch_projects_by_job_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;
    
    // Use job_id 1 for testing
    let job_id = 1;

    // Act: Simulate HTTP GET request to `/projects/job/1`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/projects/job/{}", job_id))
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Ensure response status is OK
    assert_eq!(response.status(), 200);

    // Parse and verify the response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let projects: Vec<Project> = serde_json::from_slice(&body).expect("Failed to parse response body");

    // Verify that all returned projects have the specified job_id
    for project in &projects {
        assert_eq!(project.job_id, Some(job_id), "All projects should have job_id = {}", job_id);
    }
    
    println!("Fetched projects for job {}: {:?}", job_id, projects);
}

#[tokio::test]
async fn test_fetch_projects_by_skill_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;
    
    // Use skill_id 1 for testing
    let skill_id = 10;

    // Act: Simulate HTTP GET request to `/projects/skill/1`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/projects/skill/{}", skill_id))
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Ensure response status is OK
    assert_eq!(response.status(), 200);

    // Parse and verify the response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let projects: Vec<Project> = serde_json::from_slice(&body).expect("Failed to parse response body");

    // We can't easily assert that all projects have this skill without deeper inspection
    // because the skills are inside each project object
    assert!(!projects.is_empty(), "Should return at least one project with skill ID {}", skill_id);
    
    // Verify at least one project has the skill with the specified ID
    let has_skill = projects.iter().any(|p| {
        p.skills.iter().any(|s| s.id == skill_id)
    });
    
    assert!(has_skill, "At least one project should have the skill with ID {}", skill_id);
    
    println!("Fetched projects with skill {}: {:?}", skill_id, projects);
}

#[tokio::test]
async fn test_fetch_project_by_id_not_found_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;
    
    // Use a non-existent project ID (assuming 9999 doesn't exist)
    let non_existent_id = 9999;

    // Act: Simulate HTTP GET request to `/projects/9999`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/projects/{}", non_existent_id))
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Expect a 404 Not Found response
    assert_eq!(response.status(), 404);
}

