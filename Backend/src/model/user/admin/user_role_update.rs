use crate::common::BoxError;
use crate::model::contract::task::Task;
use crate::model::user::role::Role;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserRoleUpdate {
    pool: Arc<PgPool>,
    target_user_id: Uuid,
    roles: Vec<Role>,
}

impl UserRoleUpdate {
    pub fn new(pool: Arc<PgPool>, target_user_id: Uuid, roles: Vec<Role>) -> Self {
        Self {
            pool,
            target_user_id,
            roles,
        }
    }
}

#[async_trait::async_trait]
impl Task for UserRoleUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM user_roles WHERE user_id = $1")
            .bind(self.target_user_id)
            .execute(&mut *tx)
            .await?;
        for role in &self.roles {
            sqlx::query("INSERT INTO user_roles (user_id, role) VALUES ($1, $2)")
                .bind(self.target_user_id)
                .bind(role)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
