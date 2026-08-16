use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct EmailUpdate {
    pool: Arc<PgPool>,
    user_id: UserId,
    email: String,
}

impl EmailUpdate {
    pub fn new(pool: Arc<PgPool>, user_id: UserId, email: String) -> Self {
        Self {
            pool,
            user_id,
            email,
        }
    }
}

#[async_trait::async_trait]
impl Task for EmailUpdate {
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        sqlx::query("UPDATE users SET email = $2 WHERE id = $1")
            .bind(self.user_id.id())
            .bind(&self.email)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
