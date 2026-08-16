use crate::model::user::contract::user_verification::VerificationError;
use crate::model::user::id::UserId;

#[async_trait::async_trait]
pub trait ProtectedUser: Send + Sync {
    async fn unprotected(&self) -> Result<UserId, VerificationError>;
}
