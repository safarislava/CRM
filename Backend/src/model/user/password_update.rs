use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::credential::contract::hash::Hash;
use crate::model::user::contract::protected_user::ProtectedUser;
use sqlx::PgPool;
use std::sync::Arc;

pub struct PasswordUpdate {
    pool: Arc<PgPool>,
    protected_user: Box<dyn ProtectedUser>,
    new_password: Box<dyn Hash>,
}

impl PasswordUpdate {
    pub fn new(
        pool: Arc<PgPool>,
        protected_user: Box<dyn ProtectedUser>,
        new_password: Box<dyn Hash>,
    ) -> Self {
        Self { pool, protected_user, new_password }
    }
}

#[async_trait::async_trait]
impl Task for PasswordUpdate {
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        let (user, hash) = futures_util::try_join!(
            async { self.protected_user.unprotected().await.map_err(BoxError::from) },
            async { self.new_password.value().await.map_err(BoxError::from) }
        )?;
        sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
            .bind(user.id())
            .bind(hash)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
