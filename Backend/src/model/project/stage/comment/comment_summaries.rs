use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_media::CommentMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::stage::stage_id::StageId;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct CommentSummaries {
    pool: Arc<PgPool>,
    stage_id: StageId,
    before: Option<Uuid>,
}

impl CommentSummaries {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, before: Option<Uuid>) -> Self {
        Self {
            pool,
            stage_id,
            before,
        }
    }
}

#[async_trait::async_trait]
impl<M: CommentMedia> Printer<M> for CommentSummaries {
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
        let mut rows = match self.before {
            None => sqlx::query_as::<_, Row>(
                "SELECT c.id, c.text, u.username AS author, c.is_system, c.created_at, c.is_pinned \
                     FROM stage_comments c \
                     JOIN users u ON u.id = c.author_id \
                     WHERE c.project_id = $1 AND c.parent_position = $2 AND c.stage_position = $3 \
                     ORDER BY c.created_at DESC, c.id DESC \
                     LIMIT 25",
            )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .fetch_all(self.pool.as_ref())
            .await?,
            Some(before) => sqlx::query_as::<_, Row>(
                "SELECT c.id, c.text, u.username AS author, c.is_system, c.created_at, c.is_pinned \
                     FROM stage_comments c \
                     JOIN users u ON u.id = c.author_id \
                     WHERE c.project_id = $1 AND c.parent_position = $2 AND c.stage_position = $3 \
                     AND (c.created_at, c.id) < \
                         (SELECT created_at, id FROM stage_comments WHERE id = $4) \
                     ORDER BY c.created_at DESC, c.id DESC \
                     LIMIT 25",
            )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(before)
            .fetch_all(self.pool.as_ref())
            .await?,
        };
        rows.reverse();
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
