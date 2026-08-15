use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_media::CommentMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::stage::Stage;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct PinnedCommentSummaries {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl PinnedCommentSummaries {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl<M: CommentMedia> Printer<M> for PinnedCommentSummaries {
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
            text: String,
            author: String,
            is_system: bool,
            created_at: DateTime<Utc>,
            is_pinned: bool,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT c.id, c.text, u.username AS author, c.is_system, c.created_at, c.is_pinned \
             FROM stage_comments c \
             JOIN users u ON u.id = c.author_id \
             WHERE c.project_id = $1 AND c.parent_position = $2 AND c.stage_position = $3 \
             AND c.is_pinned = TRUE \
             ORDER BY c.created_at, c.id",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .fetch_all(self.pool.as_ref())
        .await?;

        for r in rows {
            media.add_comment(
                r.id,
                &r.text,
                &r.author,
                r.is_system,
                r.created_at,
                r.is_pinned,
            );
        }

        Ok(())
    }
}
