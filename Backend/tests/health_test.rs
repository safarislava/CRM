mod common;

use actix_web::test::{self, TestRequest};
use dailycrm::endpoint::health::HealthStatus;

#[actix_web::test]
async fn test_health_endpoint_returns_json_status() {
    let app = init_test_app!();

    let req = TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success() || resp.status().as_u16() == 503);
    let body: HealthStatus = test::read_body_json(resp).await;
    assert!(body.database == "connected" || body.database == "disconnected");
}
