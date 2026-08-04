use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::advance_payment_confirmation::AdvancePaymentConfirmation;
use crate::model::project::advance_payment_confirmation_text::AdvancePaymentConfirmationText;
use crate::model::project::stage::Stage;
use crate::model::project::stage_advance_payment_confirmed_receipt::StageAdvancePaymentConfirmedReceipt;
use crate::model::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedAdvancePaymentConfirmation {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    confirmed: bool,
}

impl LoggedAdvancePaymentConfirmation {
    pub fn new(pool: Arc<PgPool>, stage: Stage, user: User, confirmed: bool) -> Self {
        Self {
            pool,
            stage,
            user,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedAdvancePaymentConfirmation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StageAdvancePaymentConfirmedReceipt::new(self.pool.clone(), self.stage.clone())
            .value()
            .await?;
        AdvancePaymentConfirmation::new(self.pool.clone(), self.stage.clone(), self.confirmed)
            .perform()
            .await?;
        if old != Some(self.confirmed) {
            let text = AdvancePaymentConfirmationText::new(self.confirmed).text();
            let _ = SystemCommentCreation::new(
                self.pool.clone(),
                self.stage.clone(),
                self.user.clone(),
                text,
            )
            .perform()
            .await;
        }
        Ok(())
    }
}
