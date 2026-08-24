use crate::model::contract::box_error::BoxError;
use crate::model::contract::deadline_media::DeadlineMedia;
use crate::model::contract::printer::Printer;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::model::project::id::ProjectId;
use crate::model::project::stage::id::StageId;

pub struct Deadlines {
    pool: Arc<PgPool>,
}

impl Deadlines {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<M: DeadlineMedia> Printer<M> for Deadlines {
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_id: Uuid,
            parent_position: i32,
            position: i32,
            title: String,
            deadline: Option<DateTime<Utc>>,
            completed: bool,
            project_title: String,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT s.project_id, s.parent_position, s.position, s.title, s.deadline,
                    s.completed, p.title AS project_title
             FROM detailed_stages s
             JOIN projects p ON p.id = s.project_id
             WHERE s.deadline IS NOT NULL
             ORDER BY s.deadline",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        for r in rows {
            let stage_id =
                StageId::new_substage(ProjectId::from(r.project_id), r.parent_position, r.position);
            media.add_deadline(
                stage_id,
                &r.title,
                r.deadline,
                r.completed,
                &r.project_title,
            );
        }
        Ok(())
    }
}
