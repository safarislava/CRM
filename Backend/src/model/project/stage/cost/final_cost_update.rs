use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::stage_id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct FinalCostUpdate {
    pool: Arc<PgPool>,
    stage_id: StageId,
    cost: Option<i32>,
}

impl FinalCostUpdate {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, cost: Option<i32>) -> Self {
        Self {
            pool,
            stage_id,
            cost,
        }
    }
}

#[async_trait::async_trait]
impl Task for FinalCostUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET final_cost = $4 WHERE project_id = $1 AND parent_position = $2 AND position = $3")
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(self.cost)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
