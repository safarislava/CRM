use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::attachment::AttachmentId;
use crate::model::project::project::ProjectId;
use crate::model::project::stage::StageId;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AttachmentReceipt {
    pool: Arc<PgPool>,
    attachment_id: AttachmentId,
}

impl AttachmentReceipt {
    pub fn new(pool: Arc<PgPool>, attachment_id: AttachmentId) -> Self {
        Self {
            pool,
            attachment_id,
        }
    }
}

#[async_trait::async_trait]
impl Value<Option<(String, StageId, bool)>> for AttachmentReceipt {
    async fn value(&self) -> Result<Option<(String, StageId, bool)>, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_id: Uuid,
            parent_position: i32,
            stage_position: i32,
            filename: String,
            is_act: bool,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT project_id, parent_position, stage_position, filename, is_act FROM attachments WHERE id = $1",
        )
            .bind(self.attachment_id.id())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.map(|r| {
            let stage_id = StageId::new_substage(
                ProjectId::new(r.project_id),
                r.parent_position,
                r.stage_position,
            );
            (r.filename, stage_id, r.is_act)
        }))
    }
}
