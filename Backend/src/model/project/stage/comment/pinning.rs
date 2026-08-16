use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::comment::id::CommentId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct CommentPinning {
    pool: Arc<PgPool>,
    comment_id: CommentId,
    pinned: bool,
}

impl CommentPinning {
    pub fn new(pool: Arc<PgPool>, comment_id: CommentId, pinned: bool) -> Self {
        Self {
            pool,
            comment_id,
            pinned,
        }
    }
}

#[async_trait::async_trait]
impl Task for CommentPinning {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let rows_affected = sqlx::query("UPDATE stage_comments SET is_pinned = $1 WHERE id = $2")
            .bind(self.pinned)
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
