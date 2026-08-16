use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageTitleReceipt {
    pool: Arc<PgPool>,
    stage_id: StageId,
}

impl StageTitleReceipt {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId) -> Self {
        Self { pool, stage_id }
    }
}

#[async_trait::async_trait]
impl Value<Option<String>> for StageTitleReceipt {
    async fn value(&self) -> Result<Option<String>, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            title: String,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT title FROM stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.map(|r| r.title))
    }
}
