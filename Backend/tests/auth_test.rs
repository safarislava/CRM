mod common;

use actix_web::cookie::Cookie;
use actix_web::test::{self, TestRequest};
use serde_json::json;

#[actix_web::test]
async fn test_login_returns_not_found_for_nonexistent_user() {
    let app = init_test_app!();

    let payload = json!({
        "username": "nonexistent_user_xyz",
        "password": "Password123!"
    });

    let req = TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 404);
}

#[actix_web::test]
async fn test_refresh_returns_unauthorized_when_no_cookie() {
    let app = init_test_app!();

    let req = TestRequest::post().uri("/api/auth/refresh").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_refresh_returns_unauthorized_for_invalid_cookie() {
    let app = init_test_app!();

    let req = TestRequest::post()
        .uri("/api/auth/refresh")
        .cookie(Cookie::new("refresh_token", "invalid_jwt_token_payload"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_logout_clears_refresh_token_cookie() {
    let app = init_test_app!();

    let req = TestRequest::post().uri("/api/auth/logout").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    let cookie_header = resp.headers().get("set-cookie");
    assert!(cookie_header.is_some());
    let cookie_str = cookie_header.unwrap().to_str().unwrap();
    assert!(cookie_str.contains("refresh_token="));
}
