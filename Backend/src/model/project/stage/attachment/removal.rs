use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::attachment::id::AttachmentId;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AttachmentRemoval {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    attachment_id: AttachmentId,
}

impl AttachmentRemoval {
    pub fn new(pool: Arc<PgPool>, storage: Arc<Storage>, attachment_id: AttachmentId) -> Self {
        Self {
            pool,
            storage,
            attachment_id,
        }
    }
}

#[async_trait::async_trait]
impl Task for AttachmentRemoval {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let _ = self
            .storage
            .delete(&self.attachment_id.id().to_string())
            .await;
        let result = sqlx::query("DELETE FROM attachments WHERE id = $1")
            .bind(self.attachment_id.id())
            .execute(self.pool.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Err(BoxError::from("attachment_id not found"));
        }
        Ok(())
    }
}
