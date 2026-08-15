use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::comment::CommentId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct CommentRemoval {
    pool: Arc<PgPool>,
    comment_id: CommentId,
}

impl CommentRemoval {
    pub fn new(pool: Arc<PgPool>, comment_id: CommentId) -> Self {
        Self { pool, comment_id }
    }
}

#[async_trait::async_trait]
impl Task for CommentRemoval {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let rows_affected = sqlx::query("DELETE FROM stage_comments WHERE id = $1")
            .bind(self.comment_id.id())
            .execute(self.pool.as_ref())
            .await?
            .rows_affected();
        if rows_affected == 0 {
            return Err("Comment not found".into());
        }
        Ok(())
    }
}
