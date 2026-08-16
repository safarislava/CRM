use crate::model::contract::attachment_media::AttachmentMedia;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::project::stage::stage_id::StageId;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AttachmentSummaries {
    pool: Arc<PgPool>,
    stage_id: StageId,
}

impl AttachmentSummaries {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId) -> Self {
        Self { pool, stage_id }
    }
}

#[async_trait::async_trait]
impl<M: AttachmentMedia> Printer<M> for AttachmentSummaries {
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
            project_id: Uuid,
            parent_position: i32,
            stage_position: i32,
            filename: String,
            mime_type: String,
            size_bytes: i64,
            created_at: DateTime<Utc>,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT id, project_id, parent_position, stage_position, filename, mime_type, size_bytes, created_at \
             FROM attachments \
             WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND is_act = FALSE \
             ORDER BY created_at",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .fetch_all(self.pool.as_ref())
        .await?;
        for row in rows {
            let download_url = if row.parent_position == 0 {
                format!(
                    "/api/projects/{}/stages/{}/attachments/{}/download",
                    row.project_id, row.stage_position, row.id
                )
            } else {
                format!(
                    "/api/projects/{}/stages/{}/sub/{}/attachments/{}/download",
                    row.project_id, row.parent_position, row.stage_position, row.id
                )
            };
            media.add_attachment(
                row.id,
                &row.filename,
                &row.mime_type,
                row.size_bytes,
                row.created_at,
                &download_url,
            );
        }
        Ok(())
    }
}
