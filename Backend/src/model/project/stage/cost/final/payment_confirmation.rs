use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct FinalPaymentConfirmation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    confirmed: bool,
}

impl FinalPaymentConfirmation {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, confirmed: bool) -> Self {
        Self {
            pool,
            stage_id,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for FinalPaymentConfirmation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "UPDATE stages SET final_confirmed = $4 WHERE project_id = $1 AND parent_position = $2 AND position = $3",
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
