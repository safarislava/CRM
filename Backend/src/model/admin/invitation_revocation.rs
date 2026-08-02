use crate::common::BoxError;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct InvitationRevocation {
    pool: Arc<PgPool>,
    token: Uuid,
}

impl InvitationRevocation {
    pub fn new(pool: Arc<PgPool>, token: Uuid) -> Self {
        Self { pool, token }
    }
}

#[async_trait::async_trait]
impl Task for InvitationRevocation {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("DELETE FROM invites WHERE token = $1")
            .bind(self.token)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
