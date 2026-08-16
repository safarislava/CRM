use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::notification::notification_enqueue::NotificationEnqueue;
use crate::model::project::stage::gip::gip_confirmation::GipConfirmation;
use crate::model::project::stage::stage_id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotifiedGipConfirmation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    confirmed: bool,
}

impl NotifiedGipConfirmation {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, confirmed: bool) -> Self {
        Self {
            pool,
            stage_id,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for NotifiedGipConfirmation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        GipConfirmation::new(self.pool.clone(), self.stage_id.clone(), self.confirmed)
            .perform()
            .await?;
        if self.confirmed {
            NotificationEnqueue::new(self.pool.clone(), self.stage_id.clone(), "work_complete")
                .perform()
                .await?;
        }
        Ok(())
    }
}
