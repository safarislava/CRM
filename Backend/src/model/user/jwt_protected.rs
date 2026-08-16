use crate::model::contract::value::Value;
use crate::model::session::refresh_token::RefreshToken;
use crate::model::session::user_id_receipt::UserIdReceipt;
use crate::model::user::contract::protected_user::ProtectedUser;
use crate::model::user::contract::user_verification::VerificationError;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct JwtProtectedUser {
    pool: Arc<PgPool>,
    refresh_token: RefreshToken,
}

impl JwtProtectedUser {
    pub fn new(pool: Arc<PgPool>, refresh_token: RefreshToken) -> Self {
        Self {
            pool,
            refresh_token,
        }
    }
}

#[async_trait::async_trait]
impl ProtectedUser for JwtProtectedUser {
    async fn unprotected(&self) -> Result<UserId, VerificationError> {
        match UserIdReceipt::new(self.pool.clone(), self.refresh_token.id())
            .value()
            .await
        {
            Ok(Some(id)) => Ok(UserId::new(id)),
            Ok(None) => Err(VerificationError::Wrong),
            Err(_) => Err(VerificationError::Internal),
        }
    }
}
