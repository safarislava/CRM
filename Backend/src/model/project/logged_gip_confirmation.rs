use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::gip_confirmation_text::GipConfirmationText;
use crate::model::project::notified_gip_confirmation::NotifiedGipConfirmation;
use crate::model::project::stage::StageId;
use crate::model::project::stage_gip_confirmed_receipt::StageGipConfirmedReceipt;
use crate::model::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedGipConfirmation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    confirmed: bool,
}

impl LoggedGipConfirmation {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, user_id: UserId, confirmed: bool) -> Self {
        Self {
            pool,
            stage_id,
            user_id,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedGipConfirmation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StageGipConfirmedReceipt::new(self.pool.clone(), self.stage_id.clone())
            .value()
            .await?;
        NotifiedGipConfirmation::new(self.pool.clone(), self.stage_id.clone(), self.confirmed)
            .perform()
            .await?;
        if old != Some(self.confirmed) {
            let text = GipConfirmationText::new(self.confirmed).text();
            let _ = SystemCommentCreation::new(
                self.pool.clone(),
                self.stage_id.clone(),
                self.user_id.clone(),
                text,
            )
            .perform()
            .await;
        }
        Ok(())
    }
}
