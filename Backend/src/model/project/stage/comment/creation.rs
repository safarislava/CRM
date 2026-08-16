use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct CommentCreation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    author: UserId,
    text: String,
}

impl CommentCreation {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, author: UserId, text: String) -> Self {
        Self {
            pool,
            stage_id,
            author,
            text,
        }
    }
}

#[async_trait::async_trait]
impl Task for CommentCreation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "INSERT INTO stage_comments(project_id, parent_position, stage_position, author_id, text) \
             VALUES ($1, $2, $3, $4, $5)",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .bind(self.author.id())
        .bind(&self.text)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
