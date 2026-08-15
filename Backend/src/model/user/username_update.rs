use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::credential::contract::username::Username;
use crate::model::user::user::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct UsernameUpdate {
    pool: Arc<PgPool>,
    user_id: UserId,
    new_username: Box<dyn Username>,
}

impl UsernameUpdate {
    pub fn new(pool: Arc<PgPool>, user_id: UserId, new_username: impl Username) -> Self {
        Self {
            pool,
            user_id,
            new_username: Box::new(new_username),
        }
    }
}

#[async_trait::async_trait]
impl Task for UsernameUpdate {
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        let result = sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(self.user_id.id())
            .bind(self.new_username.value().await?)
            .execute(self.pool.as_ref())
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
