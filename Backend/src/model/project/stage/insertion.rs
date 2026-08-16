use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageInsertion {
    pool: Arc<PgPool>,
    project_id: ProjectId,
    position: i32,
    title: String,
}

impl StageInsertion {
    pub fn new(pool: Arc<PgPool>, project_id: ProjectId, position: i32, title: String) -> Self {
        Self {
            pool,
            project_id,
            position,
            title,
        }
    }
}

#[async_trait::async_trait]
impl Task for StageInsertion {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query(
            "UPDATE stages SET position = -position \
             WHERE project_id = $1 AND parent_position = 0 AND position >= $2",
        )
        .bind(self.project_id.id())
        .bind(self.position)
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "UPDATE stages SET parent_position = -parent_position \
             WHERE project_id = $1 AND parent_position >= $2",
        )
        .bind(self.project_id.id())
        .bind(self.position)
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "UPDATE stages SET position = -position + 1 \
             WHERE project_id = $1 AND parent_position = 0 AND position < 0",
        )
        .bind(self.project_id.id())
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "UPDATE stages SET parent_position = -parent_position + 1 \
             WHERE project_id = $1 AND parent_position < 0",
        )
        .bind(self.project_id.id())
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "INSERT INTO stages(project_id, parent_position, position, title) VALUES ($1, 0, $2, $3)",
        )
            .bind(self.project_id.id())
        .bind(self.position)
        .bind(&self.title)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(())
    }
}
