use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::stage::comment::rename_text::RenameText;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::rename::StageRename;
use crate::model::project::stage::title_receipt::StageTitleReceipt;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedStageRename {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    title: String,
}

impl LoggedStageRename {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, user_id: UserId, title: String) -> Self {
        Self {
            pool,
            stage_id,
            user_id,
            title,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedStageRename {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StageTitleReceipt::new(self.pool.clone(), self.stage_id.clone())
            .value()
            .await?;
        StageRename::new(self.pool.clone(), self.stage_id.clone(), self.title.clone())
            .perform()
            .await?;
        if let Some(old_title) = old {
            if old_title != self.title {
                let text = RenameText::new(old_title, self.title.clone()).text();
                let _ = SystemCommentCreation::new(
                    self.pool.clone(),
                    self.stage_id.clone(),
                    self.user_id.clone(),
                    text,
                )
                .perform()
                .await;
            }
        }
        Ok(())
    }
}
