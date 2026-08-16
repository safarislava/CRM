use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::contract::project_media::ProjectMedia;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct ProjectSummaries {
    pool: Arc<PgPool>,
}

impl ProjectSummaries {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<M: ProjectMedia> Printer<M> for ProjectSummaries {
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
            title: String,
            updated_at: DateTime<Utc>,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT id, title, updated_at FROM projects ORDER BY updated_at DESC",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        for r in rows {
            media.add_project(r.id, &r.title, r.updated_at);
        }
        Ok(())
    }
}
