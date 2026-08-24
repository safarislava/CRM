mod common;

use actix_web::test::{self, TestRequest};

#[actix_web::test]
async fn test_get_comments_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get()
        .uri("/api/projects/1/stages/1/comments")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_get_attachments_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get()
        .uri("/api/projects/1/stages/1/attachments")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_get_acts_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get()
        .uri("/api/projects/1/stages/1/acts")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}
