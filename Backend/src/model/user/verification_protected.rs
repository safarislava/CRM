use crate::model::user::contract::protected_user::ProtectedUser;
use crate::model::user::contract::user_verification::{UserVerification, VerificationError};
use crate::model::user::id::UserId;

pub struct VerificationProtectedUser {
    user_id: UserId,
    verification: Box<dyn UserVerification>,
}

impl VerificationProtectedUser {
    pub fn new(user_id: UserId, verification: impl UserVerification) -> Self {
        Self {
            user_id,
            verification: Box::new(verification),
        }
    }
}

#[async_trait::async_trait]
impl ProtectedUser for VerificationProtectedUser {
    async fn unprotected(&self) -> Result<UserId, VerificationError> {
        match self.verification.status().await {
            Ok(_) => Ok(self.user_id),
            Err(e) => Err(e),
        }
    }
}
