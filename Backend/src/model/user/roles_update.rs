use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::user::role::Role;
use crate::model::user::user::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct RolesUpdate {
    pool: Arc<PgPool>,
    user_id: UserId,
    roles: Vec<Role>,
}

impl RolesUpdate {
    pub fn new(pool: Arc<PgPool>, user_id: UserId, roles: Vec<Role>) -> Self {
        Self {
            pool,
            user_id,
            roles,
        }
    }
}

#[async_trait::async_trait]
impl Task for RolesUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM user_roles WHERE user_id = $1")
            .bind(self.user_id.id())
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO user_roles (user_id, role) SELECT $1, UNNEST($2)")
            .bind(self.user_id.id())
            .bind(&self.roles as &[Role])
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}
