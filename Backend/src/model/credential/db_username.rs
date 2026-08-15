use crate::model::credential::contract::username::{Username, UsernameError};
use crate::model::user::user::UserId;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

pub struct DbUsername {
    pool: Arc<PgPool>,
    user_id: UserId,
}

impl DbUsername {
    pub fn new(pool: Arc<PgPool>, user_id: UserId) -> Self {
        Self { pool, user_id }
    }
}

#[async_trait]
impl Username for DbUsername {
    async fn value(&self) -> Result<String, UsernameError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            username: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT username FROM users WHERE id = $1")
            .bind(self.user_id.id())
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(|e| UsernameError::Internal(Box::from(e)))?;
        row.map(|r| r.username).ok_or(UsernameError::NotFound)
    }
}
