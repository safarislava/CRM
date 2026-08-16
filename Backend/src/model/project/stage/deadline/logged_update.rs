use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::deadline::change_text::DeadlineChangeText;
use crate::model::project::stage::deadline::receipt::StageDeadlineReceipt;
use crate::model::project::stage::deadline::update::DeadlineUpdate;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedDeadlineUpdate {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    deadline: Option<DateTime<Utc>>,
}

impl LoggedDeadlineUpdate {
    pub fn new(
        pool: Arc<PgPool>,
        stage_id: StageId,
        user_id: UserId,
        deadline: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            pool,
            stage_id,
            user_id,
            deadline,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedDeadlineUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StageDeadlineReceipt::new(self.pool.clone(), self.stage_id.clone())
            .value()
            .await?;
        DeadlineUpdate::new(self.pool.clone(), self.stage_id.clone(), self.deadline)
            .perform()
            .await?;
        if let Some(old_date) = old {
            if self.deadline != Some(old_date) {
                let text = DeadlineChangeText::new(old_date, self.deadline).text();
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
