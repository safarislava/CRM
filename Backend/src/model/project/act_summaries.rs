use crate::model::contract::act_media::ActMedia;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::project::stage::Stage;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct ActSummaries {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl ActSummaries {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl<M: ActMedia> Printer<M> for ActSummaries {
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
             WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND is_act = TRUE \
             ORDER BY created_at",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .fetch_all(self.pool.as_ref())
        .await?;
        for row in rows {
            let download_url = if row.parent_position == 0 {
                format!(
                    "/api/projects/{}/stages/{}/act/{}/download",
                    row.project_id, row.stage_position, row.id
                )
            } else {
                format!(
                    "/api/projects/{}/stages/{}/sub/{}/act/{}/download",
                    row.project_id, row.parent_position, row.stage_position, row.id
                )
            };
            media.add_act(
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