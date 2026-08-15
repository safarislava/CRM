use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::final_payment_confirmation::FinalPaymentConfirmation;
use crate::model::project::final_payment_confirmation_text::FinalPaymentConfirmationText;
use crate::model::project::stage::StageId;
use crate::model::project::stage_final_payment_confirmed_receipt::StageFinalPaymentConfirmedReceipt;
use crate::model::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedFinalPaymentConfirmation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    confirmed: bool,
}

impl LoggedFinalPaymentConfirmation {
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
impl Task for LoggedFinalPaymentConfirmation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StageFinalPaymentConfirmedReceipt::new(self.pool.clone(), self.stage_id.clone())
            .value()
            .await?;
        FinalPaymentConfirmation::new(self.pool.clone(), self.stage_id.clone(), self.confirmed)
            .perform()
            .await?;
        if old != Some(self.confirmed) {
            let text = FinalPaymentConfirmationText::new(self.confirmed).text();
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
