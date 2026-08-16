use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::cost::advance::payment_confirmation::AdvancePaymentConfirmation;
use crate::model::project::stage::cost::advance::payment_confirmation_text::AdvancePaymentConfirmationText;
use crate::model::project::stage::cost::advance::payment_confirmed_receipt::StageAdvancePaymentConfirmedReceipt;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedAdvancePaymentConfirmation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    confirmed: bool,
}

impl LoggedAdvancePaymentConfirmation {
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
impl Task for LoggedAdvancePaymentConfirmation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old =
            StageAdvancePaymentConfirmedReceipt::new(self.pool.clone(), self.stage_id.clone())
                .value()
                .await?;
        AdvancePaymentConfirmation::new(self.pool.clone(), self.stage_id.clone(), self.confirmed)
            .perform()
            .await?;
        if old != Some(self.confirmed) {
            let text = AdvancePaymentConfirmationText::new(self.confirmed).text();
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
