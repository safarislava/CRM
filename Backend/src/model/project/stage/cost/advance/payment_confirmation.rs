use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AdvancePaymentConfirmation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    confirmed: bool,
}

impl AdvancePaymentConfirmation {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, confirmed: bool) -> Self {
        Self {
            pool,
            stage_id,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for AdvancePaymentConfirmation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "UPDATE stages SET advance_confirmed = $4 WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .bind(self.confirmed)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
