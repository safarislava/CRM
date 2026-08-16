use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotificationsUpdate {
    pool: Arc<PgPool>,
    user_id: UserId,
    enabled: bool,
}

impl NotificationsUpdate {
    pub fn new(pool: Arc<PgPool>, user_id: UserId, enabled: bool) -> Self {
        Self {
            pool,
            user_id,
            enabled,
        }
    }
}

#[async_trait::async_trait]
impl Task for NotificationsUpdate {
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        sqlx::query("UPDATE users SET notifications_enabled = $2 WHERE id = $1")
            .bind(self.user_id.id())
            .bind(self.enabled)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
