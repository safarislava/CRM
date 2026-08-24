mod common;

use actix_web::test::{self, TestRequest};
use serde_json::json;
use uuid::Uuid;

#[actix_web::test]
async fn test_users_me_returns_unauthorized_without_bearer_token() {
    let app = init_test_app!();

    let req = TestRequest::get().uri("/api/users/me").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_users_me_returns_unauthorized_for_invalid_jwt_bearer() {
    let app = init_test_app!();

    let req = TestRequest::get()
        .uri("/api/users/me")
        .insert_header(("Authorization", "Bearer invalid.jwt.token"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_user_creation_fails_for_invalid_invite_token() {
    let app = init_test_app!();

    let payload = json!({
        "username": "new_test_user",
        "password": "Password123!",
        "invite_token": Uuid::new_v4().to_string(),
        "email": "user@example.com"
    });

    let req = TestRequest::post()
        .uri("/api/users")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 403);
}
