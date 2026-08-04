use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::gip_confirmation::GipConfirmation;
use crate::model::project::stage::Stage;
use crate::model::notification::notification_enqueue::NotificationEnqueue;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotifiedGipConfirmation {
    pool: Arc<PgPool>,
    stage: Stage,
    confirmed: bool,
}

impl NotifiedGipConfirmation {
    pub fn new(pool: Arc<PgPool>, stage: Stage, confirmed: bool) -> Self {
        Self {
            pool,
            stage,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for NotifiedGipConfirmation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        GipConfirmation::new(self.pool.clone(), self.stage.clone(), self.confirmed)
            .perform()
            .await?;
        if self.confirmed {
            NotificationEnqueue::new(self.pool.clone(), self.stage.clone(), "work_complete")
                .perform()
                .await?;
        }
        Ok(())
    }
}
