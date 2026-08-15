use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::final_cost_change_text::FinalCostChangeText;
use crate::model::project::final_cost_update::FinalCostUpdate;
use crate::model::project::stage::StageId;
use crate::model::project::stage_final_cost_receipt::StageFinalCostReceipt;
use crate::model::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedFinalCostUpdate {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    cost: Option<i32>,
}

impl LoggedFinalCostUpdate {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, user_id: UserId, cost: Option<i32>) -> Self {
        Self {
            pool,
            stage_id,
            user_id,
            cost,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedFinalCostUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StageFinalCostReceipt::new(self.pool.clone(), self.stage_id.clone())
            .value()
            .await?;
        FinalCostUpdate::new(self.pool.clone(), self.stage_id.clone(), self.cost)
            .perform()
            .await?;
        if let Some(old_cost) = old {
            if self.cost != Some(old_cost) {
                let text = FinalCostChangeText::new(old_cost, self.cost).text();
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
