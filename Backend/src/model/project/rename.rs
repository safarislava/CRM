use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct ProjectRename {
    pool: Arc<PgPool>,
    project_id: ProjectId,
    title: String,
}

impl ProjectRename {
    pub fn new(pool: Arc<PgPool>, project_id: ProjectId, title: String) -> Self {
        Self {
            pool,
            project_id,
            title,
        }
    }
}

#[async_trait::async_trait]
impl Task for ProjectRename {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let result = sqlx::query("UPDATE projects SET title = $2 WHERE id = $1")
            .bind(self.project_id.id())
            .bind(&self.title)
            .execute(self.pool.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Err(BoxError::from(sqlx::Error::RowNotFound));
        }
        Ok(())
    }
}
