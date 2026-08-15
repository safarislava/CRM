use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::StageId;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageDeadlineReceipt {
    pool: Arc<PgPool>,
    stage_id: StageId,
}

impl StageDeadlineReceipt {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId) -> Self {
        Self { pool, stage_id }
    }
}

#[async_trait::async_trait]
impl Value<Option<DateTime<Utc>>> for StageDeadlineReceipt {
    async fn value(&self) -> Result<Option<DateTime<Utc>>, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            deadline: Option<DateTime<Utc>>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT deadline FROM stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.and_then(|r| r.deadline))
    }
}
