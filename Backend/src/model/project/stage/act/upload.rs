use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::contract::file::File;
use crate::model::project::stage::id::StageId;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct ActUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage_id: StageId,
    file: Arc<dyn File>,
}

impl ActUpload {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        stage_id: StageId,
        file: Arc<dyn File>,
    ) -> Self {
        Self {
            pool,
            storage,
            stage_id,
            file,
        }
    }
}

#[async_trait::async_trait]
impl Task for ActUpload {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let id = Uuid::new_v4();
        self.file
            .upload_to(self.storage.as_ref(), &id.to_string())
            .await?;
        sqlx::query(
            "INSERT INTO attachments(id, project_id, parent_position, stage_position, filename, mime_type, size_bytes, is_act)
             VALUES ($1, $2, $3, $4, $5, $6, $7, true)",
        )
        .bind(id)
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .bind(self.file.name())
        .bind(self.file.media_type())
        .bind(self.file.size_bytes())
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
