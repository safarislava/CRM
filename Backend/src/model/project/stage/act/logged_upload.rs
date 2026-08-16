use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::project::contract::file::File;
use crate::model::project::file_content::FileContent;
use crate::model::project::stage::act::notified_upload::NotifiedActUpload;
use crate::model::project::stage::act::upload_text::ActUploadText;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedActUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage_id: StageId,
    user_id: UserId,
    file: FileContent,
}

impl LoggedActUpload {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        stage_id: StageId,
        user_id: UserId,
        file: FileContent,
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
impl Task for LoggedActUpload {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        NotifiedActUpload::new(
            self.pool.clone(),
            self.storage.clone(),
            self.stage_id.clone(),
            self.file.clone(),
        )
        .perform()
        .await?;
        let text = ActUploadText::new(self.file.name().to_string()).text();
        let _ = SystemCommentCreation::new(
            self.pool.clone(),
            self.stage_id.clone(),
            self.user_id.clone(),
            text,
        )
        .perform()
        .await;
        Ok(())
    }
}
