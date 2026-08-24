mod common;

use actix_web::test::{self, TestRequest};
use serde_json::json;

#[actix_web::test]
async fn test_get_projects_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get().uri("/api/projects").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_get_deadlines_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get()
        .uri("/api/projects/deadlines")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_create_project_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let payload = json!({
        "title": "New Test Project"
    });

    let req = TestRequest::post()
        .uri("/api/projects")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}
