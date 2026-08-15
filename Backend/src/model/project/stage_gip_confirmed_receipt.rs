use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageGipConfirmedReceipt {
    pool: Arc<PgPool>,
    stage_id: StageId,
}

impl StageGipConfirmedReceipt {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId) -> Self {
        Self { pool, stage_id }
    }
}

#[async_trait::async_trait]
impl Value<Option<bool>> for StageGipConfirmedReceipt {
    async fn value(&self) -> Result<Option<bool>, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            gip_confirmed: bool,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT gip_confirmed FROM stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.map(|r| r.gip_confirmed))
    }
}
