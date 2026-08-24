use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::project::contract::file::File;
use crate::model::project::stage::attachment::upload::AttachmentUpload;
use crate::model::project::stage::attachment::upload_text::AttachmentUploadText;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct LoggedAttachmentUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage_id: StageId,
    user_id: UserId,
    file: Arc<dyn File>,
}

impl LoggedAttachmentUpload {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        stage_id: StageId,
        user_id: UserId,
        file: Arc<dyn File>,
    ) -> Self {
        Self {
            pool,
            storage,
            stage_id,
            user_id,
            file,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedAttachmentUpload {
    type Output = Uuid;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let id = AttachmentUpload::new(
            self.pool.clone(),
            self.storage.clone(),
            self.stage_id,
            self.file.clone(),
        )
        .perform()
        .await?;
        let text = AttachmentUploadText::new(self.file.name().to_string()).text();
        let _ = SystemCommentCreation::new(self.pool.clone(), self.stage_id, self.user_id, text)
            .perform()
            .await;
        Ok(id)
    }
}
