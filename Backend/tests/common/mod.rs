#[macro_export]
macro_rules! init_test_app {
    () => {{
        dotenvy::dotenv().ok();
        let db_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => return,
        };
        let pool = match tokio::time::timeout(
            std::time::Duration::from_secs(1),
            sqlx::PgPool::connect(&db_url),
        )
        .await
        {
            Ok(Ok(pool)) => std::sync::Arc::new(pool),
            _ => return,
        };
        let storage = std::sync::Arc::new(dailycrm::storage::Storage::from_env().await);
        let mailer = std::sync::Arc::new(dailycrm::mail::Mailer::from_env());
        let state = actix_web::web::Data::new(dailycrm::state::AppState {
            pool,
            storage,
            mailer,
            project_cache: dailycrm::model::cache::memory_cache::MemoryCache::new(),
            stage_cache: dailycrm::model::cache::memory_cache::MemoryCache::new(),
            user_cache: dailycrm::model::cache::memory_cache::MemoryCache::new(),
        });
        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(state.clone())
                .wrap(dailycrm::cors::rules())
                .configure(dailycrm::routes::configure),
        )
        .await
    }};
}
