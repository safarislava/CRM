use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::id::StageId;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;

pub struct DeadlineUpdate {
    pool: Arc<PgPool>,
    stage_id: StageId,
    deadline: Option<DateTime<Utc>>,
}

impl DeadlineUpdate {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, deadline: Option<DateTime<Utc>>) -> Self {
        Self {
            pool,
            stage_id,
            deadline,
        }
    }
}

#[async_trait::async_trait]
impl Task for DeadlineUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET deadline = $4 WHERE project_id = $1 AND parent_position = $2 AND position = $3")
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(self.deadline)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
