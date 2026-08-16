use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::contract::stage_media::StageMedia;
use crate::model::project::id::ProjectId;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct StageSummaries {
    pool: Arc<PgPool>,
    project_id: ProjectId,
}

impl StageSummaries {
    pub fn new(pool: Arc<PgPool>, project_id: ProjectId) -> Self {
        Self { pool, project_id }
    }
}

#[async_trait::async_trait]
impl<M: StageMedia> Printer<M> for StageSummaries {
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_id: Uuid,
            parent_position: i32,
            position: i32,
            title: String,
            deadline: Option<DateTime<Utc>>,
            completed: bool,
            gip_confirmed: bool,
            advance_cost: Option<i32>,
            advance_confirmed: bool,
            final_cost: Option<i32>,
            final_confirmed: bool,
            has_act: bool,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT s.project_id, s.parent_position, s.position, s.title, s.deadline, s.completed,
                    s.gip_confirmed, s.advance_cost, s.advance_confirmed, s.final_cost, s.final_confirmed,
                    EXISTS(
                       SELECT 1 FROM attachments a
                       WHERE a.project_id = s.project_id
                         AND a.parent_position = s.parent_position
                         AND a.stage_position = s.position
                         AND a.is_act = TRUE
                    ) AS has_act
             FROM detailed_stages s
             WHERE s.project_id = $1 ORDER BY s.parent_position, s.position",
        )
            .bind(self.project_id.id())
        .fetch_all(self.pool.as_ref())
        .await?;
        for r in rows {
            media.add_stage(
                r.project_id,
                r.parent_position,
                r.position,
                &r.title,
                r.deadline,
                r.completed,
                r.gip_confirmed,
                r.advance_cost,
                r.advance_confirmed,
                r.final_cost,
                r.final_confirmed,
                r.has_act,
            );
        }
        Ok(())
    }
}
