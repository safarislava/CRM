use crate::common::BoxError;
use crate::model::project::contract::json::Json;
use sqlx::PgPool;
use std::sync::Arc;

pub struct Statistics {
    pool: Arc<PgPool>,
}

impl Statistics {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Json for Statistics {
    async fn json(&self) -> Result<serde_json::Value, BoxError> {
        let total_users = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(self.pool.as_ref())
            .await?;
        let total_projects = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
            .fetch_one(self.pool.as_ref())
            .await?;
        let total_stages = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM stages")
            .fetch_one(self.pool.as_ref())
            .await?;
        let pending_invitations = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM invites WHERE used_at IS NULL AND expires_at > NOW()",
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(serde_json::json!({
            "total_users": total_users,
            "total_projects": total_projects,
            "total_stages": total_stages,
            "pending_invitations": pending_invitations,
        }))
    }
}
