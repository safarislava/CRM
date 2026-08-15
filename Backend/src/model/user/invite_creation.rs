use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::user::invite::InviteCode;
use crate::model::user::user::UserId;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct InviteCreation {
    pool: Arc<PgPool>,
    user_id: UserId,
}

impl InviteCreation {
    pub fn new(pool: Arc<PgPool>, user_id: UserId) -> Self {
        Self { pool, user_id }
    }
}

#[async_trait::async_trait]
impl Task for InviteCreation {
    type Output = InviteCode;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            token: Uuid,
        }
        let row: Row =
            sqlx::query_as("INSERT INTO invites (created_by) VALUES ($1) RETURNING token")
                .bind(self.user_id.id())
                .fetch_one(self.pool.as_ref())
                .await?;
        Ok(InviteCode::new(row.token))
    }
}
