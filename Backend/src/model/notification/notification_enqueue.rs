use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::stage_id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotificationEnqueue {
    pool: Arc<PgPool>,
    stage_id: StageId,
    notification_type: String,
}

impl NotificationEnqueue {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, notification_type: impl Into<String>) -> Self {
        Self {
            pool,
            stage_id,
            notification_type: notification_type.into(),
        }
    }
}

#[async_trait::async_trait]
impl Task for NotificationEnqueue {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "INSERT INTO notification_queue (type, project_title, stage_title)
             SELECT $4, p.title, s.title
             FROM stages s JOIN projects p ON p.id = s.project_id
             WHERE s.project_id = $1 AND s.parent_position = $2 AND s.position = $3",
        )
        .bind(self.stage_id.project_id().id())
        .bind(self.stage_id.parent_position())
        .bind(self.stage_id.position())
        .bind(&self.notification_type)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
