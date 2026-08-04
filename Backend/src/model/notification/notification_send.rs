use crate::mail::Mailer;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::notification::queued_notification::QueuedNotification;
use crate::model::notification::role_recipients::RoleRecipients;
use crate::model::project::contract::list::List;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotificationSend {
    pool: Arc<PgPool>,
    mailer: Arc<Mailer>,
    notification: QueuedNotification,
}

impl NotificationSend {
    pub fn new(pool: Arc<PgPool>, mailer: Arc<Mailer>, notification: QueuedNotification) -> Self {
        Self {
            pool,
            mailer,
            notification,
        }
    }
}

#[async_trait::async_trait]
impl Task for NotificationSend {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let (Some(role), Some(subject), Some(body)) = (
            self.notification.role(),
            self.notification.subject(),
            self.notification.body(),
        ) else {
            tracing::warn!("Queue: Notification skipped due to missing required fields (role, subject, or body)");
            return Ok(());
        };
        let emails = RoleRecipients::new(self.pool.clone(), role.clone()).items().await?;
        tracing::debug!(role = ?role, count = emails.len(), "Queue: Processing notification for role");
        let sends = emails.iter().map(|email| {
            let mailer = self.mailer.clone();
            let body = body.clone();
            async move {
                match mailer.send(email, subject, body).await {
                    Ok(_) => {
                        tracing::info!(recipient = %email, "Queue: Successfully dispatched notification email");
                    }
                    Err(err) => {
                        tracing::error!(recipient = %email, error = ?err, "Queue ERROR: Failed to send notification email");
                    }
                }
            }
        });
        futures_util::future::join_all(sends).await;
        Ok(())
    }
}
