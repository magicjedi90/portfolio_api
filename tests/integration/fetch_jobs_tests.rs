use hyper::{Request, StatusCode};
use tower::ServiceExt; // For testing axum routes
use serde_json;
use portfolio_api::models::job::Job;
use crate::integration::test_utils::setup_router_with_test_db;

#[tokio::test]
async fn test_fetch_jobs_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP GET request to `/jobs`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/jobs")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Ensure response status is OK
    assert_eq!(response.status(), StatusCode::OK);

    // Parse and verify the response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let jobs: Vec<Job> = serde_json::from_slice(&body).expect("Failed to parse response body");

    assert!(!jobs.is_empty(), "Jobs should not be empty");
    println!("Fetched jobs: {:?}", jobs);
}

#[tokio::test]
async fn test_fetch_job_by_id_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP GET request to `/jobs/10`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/jobs/10")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Make sure response status is OK
    assert_eq!(response.status(), StatusCode::OK);

    // Parse and verify the response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let job: Job = serde_json::from_slice(&body).expect("Failed to parse response body");

    assert_eq!(job.id, 10, "Expected job ID to be 10");
    println!("Fetched job: {:?}", job);
}

#[tokio::test]
async fn test_fetch_job_by_id_not_found_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;
    
    // Use a non-existent job ID (assuming 9999 doesn't exist)
    let non_existent_id = 9999;

    // Act: Simulate HTTP GET request to `/jobs/9999`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/jobs/{}", non_existent_id))
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Expect a 404 Not Found response
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_negative_job_id() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP GET request with a negative ID
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/jobs/-1")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Verify how your API handles negative IDs (should probably be 404)
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_jobs_method_not_allowed() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP POST request to a GET-only endpoint
    let response = router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/jobs")
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(
                    serde_json::json!({
                        "title": "Test Job",
                        "company": "Test Company"
                    }).to_string()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Expect a 405 Method Not Allowed response
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_jobs_path_not_found() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP GET request to a non-existent jobs endpoint
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/jobs/invalid/path")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Expect a 404 Not Found response
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_jobs_content_type() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP GET request and check the content type
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/jobs")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Check that the response has the correct content type
    let content_type = response.headers().get("content-type")
        .expect("Response should have Content-Type header");
    
    assert_eq!(
        content_type, 
        "application/json", 
        "Response Content-Type should be application/json"
    );
}