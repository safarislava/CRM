use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;

pub async fn pool() -> PgPool {
    let pool = initial_pool().await;
    accept_migrations(&pool).await;
    if user_count(&pool).await == 0 {
        generate_default_user(&pool).await;
    }
    pool
}

async fn initial_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(100)
        .min_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await
        .expect("Failed to connect to database")
}

async fn accept_migrations(pool: &PgPool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to migrate the database")
}

async fn user_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .expect("Failed to get user count")
}

async fn generate_default_user(pool: &PgPool) {
    let hashed = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)
        .expect("Failed to hash default password");
    let user_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
    )
    .bind("admin")
    .bind(&hashed)
    .fetch_one(pool)
    .await
    .expect("Failed to insert default admin user");

    sqlx::query("INSERT INTO user_roles (user_id, role) VALUES ($1, 'admin') ON CONFLICT DO NOTHING")
        .bind(user_id)
        .execute(pool)
        .await
        .expect("Failed to insert default admin role");
}