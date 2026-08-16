use std::{env, sync::OnceLock};

static JWT_SECRET: OnceLock<String> = OnceLock::new();

pub fn jwt_secret() -> &'static str {
    JWT_SECRET.get_or_init(|| {
        env::var("JWT_SECRET").unwrap_or_else(|_| "test_secret_key_1234567890_default".to_string())
    })
}
