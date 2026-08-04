use crate::model::contract::box_error::BoxError;
use crate::model::project::contract::json::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct InvitationsList {
    pool: Arc<PgPool>,
}

impl InvitationsList {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow, Serialize)]
struct InvitationItem {
    token: Uuid,
    created_by: Uuid,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

#[async_trait::async_trait]
impl Json for InvitationsList {
    async fn json(&self) -> Result<serde_json::Value, BoxError> {
        let items = sqlx::query_as::<_, InvitationItem>(
            "SELECT token, created_by, created_at, expires_at FROM invites WHERE used_at IS NULL AND expires_at > NOW() ORDER BY created_at DESC",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(serde_json::to_value(items)?)
    }
}
