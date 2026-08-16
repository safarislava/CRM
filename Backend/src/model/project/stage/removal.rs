use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageRemoval {
    pool: Arc<PgPool>,
    stage_id: StageId,
}

impl StageRemoval {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId) -> Self {
        Self { pool, stage_id }
    }
}

#[async_trait::async_trait]
impl Task for StageRemoval {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let mut transaction = self.pool.begin().await?;

        let result = sqlx::query(
            "DELETE FROM stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
        .bind(self.stage_id.project_id().id())
        .bind(self.stage_id.parent_position())
        .bind(self.stage_id.position())
        .execute(&mut *transaction)
        .await?;

        if result.rows_affected() == 0 {
            return Err(BoxError::from(sqlx::Error::RowNotFound));
        }
        sqlx::query(
            "UPDATE stages SET position = -position \
             WHERE project_id = $1 AND parent_position = $2 AND position > $3",
        )
        .bind(self.stage_id.project_id().id())
        .bind(self.stage_id.parent_position())
        .bind(self.stage_id.position())
        .execute(&mut *transaction)
        .await?;

        if self.stage_id.parent_position() == 0 {
            sqlx::query(
                "UPDATE stages SET parent_position = -parent_position \
                 WHERE project_id = $1 AND parent_position > $2",
            )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.position())
            .execute(&mut *transaction)
            .await?;
        }

        sqlx::query(
            "UPDATE stages SET position = -position - 1 \
             WHERE project_id = $1 AND parent_position = $2 AND position < 0",
        )
        .bind(self.stage_id.project_id().id())
        .bind(self.stage_id.parent_position())
        .execute(&mut *transaction)
        .await?;

        if self.stage_id.parent_position() == 0 {
            sqlx::query(
                "UPDATE stages SET parent_position = -parent_position - 1 \
                 WHERE project_id = $1 AND parent_position < 0",
            )
            .bind(self.stage_id.project_id().id())
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }
}
