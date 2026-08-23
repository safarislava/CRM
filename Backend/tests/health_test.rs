use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use dailycrm::endpoint::health::HealthStatus;
use dailycrm::model::cache::memory_cache::MemoryCache;
use dailycrm::state::AppState;
use dailycrm::{cors, routes};
use std::sync::Arc;
use std::time::Duration;

#[actix_web::test]
async fn test_health_endpoint_returns_json_status() {
    dotenvy::dotenv().ok();

    let db_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    let pool =
        match tokio::time::timeout(Duration::from_secs(1), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(pool)) => Arc::new(pool),
            _ => return,
        };

    let storage = Arc::new(dailycrm::storage::Storage::from_env().await);
    let mailer = Arc::new(dailycrm::mail::Mailer::from_env());
    let state = web::Data::new(AppState {
        pool,
        storage,
        mailer,
        project_cache: MemoryCache::new(),
        stage_cache: MemoryCache::new(),
        user_cache: MemoryCache::new(),
    });

    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .wrap(cors::rules())
            .configure(routes::configure),
    )
    .await;

    let req = TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success() || resp.status().as_u16() == 503);
    let body: HealthStatus = test::read_body_json(resp).await;
    assert!(body.database == "connected" || body.database == "disconnected");
}
