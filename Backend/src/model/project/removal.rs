use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct ProjectRemoval {
    pool: Arc<PgPool>,
    project_id: ProjectId,
}

impl ProjectRemoval {
    pub fn new(pool: Arc<PgPool>, project_id: ProjectId) -> Self {
        Self { pool, project_id }
    }
}

#[async_trait::async_trait]
impl Task for ProjectRemoval {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let result = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(self.project_id.id())
            .execute(self.pool.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Err(BoxError::from(sqlx::Error::RowNotFound));
        }
        Ok(())
    }
}
