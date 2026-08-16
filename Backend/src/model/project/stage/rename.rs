use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageRename {
    pool: Arc<PgPool>,
    stage_id: StageId,
    title: String,
}

impl StageRename {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, title: String) -> Self {
        Self {
            pool,
            stage_id,
            title,
        }
    }
}

#[async_trait::async_trait]
impl Task for StageRename {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET title = $4 WHERE project_id = $1 AND parent_position = $2 AND position = $3")
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(&self.title)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
