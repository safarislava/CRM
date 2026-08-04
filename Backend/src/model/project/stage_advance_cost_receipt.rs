use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::Stage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageAdvanceCostReceipt {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl StageAdvanceCostReceipt {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl Value<Option<i32>> for StageAdvanceCostReceipt {
    async fn value(&self) -> Result<Option<i32>, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            advance_cost: Option<i32>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT advance_cost FROM stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.and_then(|r| r.advance_cost))
    }
}
