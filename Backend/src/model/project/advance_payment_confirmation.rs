use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::Stage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AdvancePaymentConfirmation {
    pool: Arc<PgPool>,
    stage: Stage,
    confirmed: bool,
}

impl AdvancePaymentConfirmation {
    pub fn new(pool: Arc<PgPool>, stage: Stage, confirmed: bool) -> Self {
        Self {
            pool,
            stage,
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
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .bind(self.confirmed)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
