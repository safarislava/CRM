use actix_web::cookie::Cookie;
use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use dailycrm::model::cache::memory_cache::MemoryCache;
use dailycrm::state::AppState;
use dailycrm::{cors, routes};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;

#[actix_web::test]
async fn test_login_returns_not_found_for_nonexistent_user() {
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

    let req = TestRequest::post().uri("/api/auth/refresh").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_refresh_returns_unauthorized_for_invalid_cookie() {
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

    let req = TestRequest::post()
        .uri("/api/auth/refresh")
        .cookie(Cookie::new("refresh_token", "invalid_jwt_token_payload"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_logout_clears_refresh_token_cookie() {
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

    let req = TestRequest::post().uri("/api/auth/logout").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    let cookie_header = resp.headers().get("set-cookie");
    assert!(cookie_header.is_some());
    let cookie_str = cookie_header.unwrap().to_str().unwrap();
    assert!(cookie_str.contains("refresh_token="));
}
