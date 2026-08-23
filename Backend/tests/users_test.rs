use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use dailycrm::model::cache::memory_cache::MemoryCache;
use dailycrm::state::AppState;
use dailycrm::{cors, routes};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[actix_web::test]
async fn test_users_me_returns_unauthorized_without_bearer_token() {
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

    let req = TestRequest::get().uri("/api/users/me").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_users_me_returns_unauthorized_for_invalid_jwt_bearer() {
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

    let req = TestRequest::get()
        .uri("/api/users/me")
        .insert_header(("Authorization", "Bearer invalid.jwt.token"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_user_creation_fails_for_invalid_invite_token() {
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
