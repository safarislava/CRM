use crate::common::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::Stage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageFinalPaymentConfirmedReceipt {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl StageFinalPaymentConfirmedReceipt {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl Value<Option<bool>> for StageFinalPaymentConfirmedReceipt {
    async fn value(&self) -> Result<Option<bool>, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            final_confirmed: bool,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT final_confirmed FROM stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.map(|r| r.final_confirmed))
    }
}
