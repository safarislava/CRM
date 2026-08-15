use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::attachment::AttachmentId;
use crate::model::project::attachment_receipt::AttachmentReceipt;
use crate::model::project::attachment_removal::AttachmentRemoval;
use crate::model::project::attachment_removal_text::AttachmentRemovalText;
use crate::model::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::UserId;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedAttachmentRemoval {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    attachment_id: AttachmentId,
    user_id: UserId,
}

impl LoggedAttachmentRemoval {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        attachment_id: AttachmentId,
        user_id: UserId,
    ) -> Self {
        Self {
            pool,
            storage,
            attachment_id,
            user_id,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedAttachmentRemoval {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let info = AttachmentReceipt::new(self.pool.clone(), self.attachment_id.clone())
            .value()
            .await?;
        AttachmentRemoval::new(
            self.pool.clone(),
            self.storage.clone(),
            self.attachment_id.clone(),
        )
        .perform()
        .await?;
        if let Some((filename, stage, is_act)) = info {
            let text = AttachmentRemovalText::new(filename, is_act).text();
            let _ =
                SystemCommentCreation::new(self.pool.clone(), stage, self.user_id.clone(), text)
                    .perform()
                    .await;
        }
        Ok(())
    }
}
