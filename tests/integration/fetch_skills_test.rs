
use hyper::Request;
use tower::ServiceExt; // For testing axum routes
use serde_json;
use portfolio_api::models::skill::Skill;
use crate::integration::test_utils::setup_router_with_test_db;

#[tokio::test]
async fn test_fetch_skills_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP GET request to `/skills`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/skills")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Ensure response status is OK
    assert_eq!(response.status(), 200);

    // Parse and verify the response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let skills: Vec<Skill> = serde_json::from_slice(&body).expect("Failed to parse response body");

    assert!(!skills.is_empty(), "Skills should not be empty");
    println!("Fetched skills: {:?}", skills);
}

#[tokio::test]
async fn test_fetch_skill_by_id_integration() {
    // Arrange: Set up the router with test DB
    let router = setup_router_with_test_db().await;

    // Act: Simulate HTTP GET request to `/skills/1`
    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/skills/1")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Make sure response status is OK
    assert_eq!(response.status(), 200);

    // Parse and verify the response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let skill: Skill = serde_json::from_slice(&body).expect("Failed to parse response body");

    assert_eq!(skill.id, 1, "Expected skill ID to be 1");
    println!("Fetched skill: {:?}", skill);
}