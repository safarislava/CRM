use crate::common::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::rename_text::RenameText;
use crate::model::project::stage::Stage;
use crate::model::project::stage_rename::StageRename;
use crate::model::project::stage_title_receipt::StageTitleReceipt;
use crate::model::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedStageRename {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    title: String,
}

impl LoggedStageRename {
    pub fn new(pool: Arc<PgPool>, stage: Stage, user: User, title: String) -> Self {
        Self {
            pool,
            stage,
            user,
            title,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedStageRename {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StageTitleReceipt::new(self.pool.clone(), self.stage.clone())
            .value()
            .await?;
        StageRename::new(self.pool.clone(), self.stage.clone(), self.title.clone())
            .perform()
            .await?;
        if let Some(old_title) = old {
            if old_title != self.title {
                let text = RenameText::new(old_title, self.title.clone()).text();
                let _ = SystemCommentCreation::new(
                    self.pool.clone(),
                    self.stage.clone(),
                    self.user.clone(),
                    text,
                )
                .perform()
                .await;
            }
        }
        Ok(())
    }
}
