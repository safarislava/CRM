use crate::common::BoxError;
use crate::mail::Mailer;
use crate::model::contract::task::Task;
use crate::model::notification::notification_dequeue::NotificationDequeue;
use crate::model::notification::notification_send::NotificationSend;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotificationDispatch {
    pool: Arc<PgPool>,
    mailer: Arc<Mailer>,
}

impl NotificationDispatch {
    pub fn new(pool: Arc<PgPool>, mailer: Arc<Mailer>) -> Self {
        Self { pool, mailer }
    }
}

#[async_trait::async_trait]
impl Task for NotificationDispatch {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let notifications = NotificationDequeue::new(self.pool.clone()).perform().await?;
        if !notifications.is_empty() {
            tracing::info!(count = notifications.len(), "Queue: Dequeued notification(s) for dispatching");
        }
        for notification in notifications {
            NotificationSend::new(self.pool.clone(), self.mailer.clone(), notification)
                .perform()
                .await?;
        }
        Ok(())
    }
}
