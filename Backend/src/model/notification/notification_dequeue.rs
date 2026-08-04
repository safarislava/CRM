use crate::common::BoxError;
use crate::model::contract::task::Task;
use crate::model::notification::queued_notification::QueuedNotification;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotificationDequeue {
    pool: Arc<PgPool>,
}

impl NotificationDequeue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Task for NotificationDequeue {
    type Output = Vec<QueuedNotification>;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        Ok(sqlx::query_as::<_, QueuedNotification>(
            "DELETE FROM notification_queue RETURNING type, project_title, stage_title",
        )
        .fetch_all(self.pool.as_ref())
        .await?)
    }
}
