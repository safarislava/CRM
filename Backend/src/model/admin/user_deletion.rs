use crate::common::BoxError;
use crate::model::task::contract::task::Task;
use crate::model::user::admin::Admin;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserDeletion {
    pool: Arc<PgPool>,
    admin: Admin,
    target_user_id: Uuid,
}

impl UserDeletion {
    pub fn new(pool: Arc<PgPool>, admin: Admin, target_user_id: Uuid) -> Self {
        Self {
            pool,
            admin,
            target_user_id,
        }
    }
}

#[async_trait::async_trait]
impl Task for UserDeletion {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        if self.admin.user().id() == self.target_user_id {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Administrators cannot delete their own account",
            )));
        }
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(self.target_user_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
