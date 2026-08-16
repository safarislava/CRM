use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::cost::advance::change_text::AdvanceCostChangeText;
use crate::model::project::stage::cost::advance::receipt::StageAdvanceCostReceipt;
use crate::model::project::stage::cost::advance::update::AdvanceCostUpdate;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedAdvanceCostUpdate {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    cost: Option<i32>,
}

impl LoggedAdvanceCostUpdate {
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
impl Task for LoggedAdvanceCostUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StageAdvanceCostReceipt::new(self.pool.clone(), self.stage_id.clone())
            .value()
            .await?;
        AdvanceCostUpdate::new(self.pool.clone(), self.stage_id.clone(), self.cost)
            .perform()
            .await?;
        if let Some(old_cost) = old {
            if self.cost != Some(old_cost) {
                let text = AdvanceCostChangeText::new(old_cost, self.cost).text();
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
