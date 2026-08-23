mod common;

use actix_web::test::{self, TestRequest};

#[actix_web::test]
async fn test_admin_statistics_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get().uri("/api/admin/statistics").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_admin_users_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get().uri("/api/admin/users").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_admin_invitations_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get()
        .uri("/api/admin/invitations")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_admin_logs_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get().uri("/api/admin/logs").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}
