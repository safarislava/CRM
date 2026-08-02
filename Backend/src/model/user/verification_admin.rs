use crate::endpoint::api_error::ApiError;
use crate::model::user::admin::Admin;
use crate::model::user::contract::admin_access::AdminAccess;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct VerificationAdmin {
    user: User,
    pool: Arc<PgPool>,
}

impl VerificationAdmin {
    pub fn new(user: User, pool: Arc<PgPool>) -> Self {
        Self { user, pool }
    }
}

#[async_trait::async_trait]
impl AdminAccess for VerificationAdmin {
    async fn admin(&self) -> Result<Admin, ApiError> {
        let is_admin = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM user_roles WHERE user_id = $1 AND role = 'admin')",
        )
        .bind(self.user.id())
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

        if is_admin {
            Ok(Admin::new(self.user.clone()))
        } else {
            Err(ApiError::Forbidden(
                "Access denied: user is not an administrator".to_string(),
            ))
        }
    }
}
