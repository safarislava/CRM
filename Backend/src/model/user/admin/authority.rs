use crate::endpoint::api_error::ApiError;
use crate::model::user::contract::admin_access::AdminAccess;
use crate::model::user::id::UserId;

#[derive(Clone, Debug)]
pub struct AdminAuthority {
    user_id: UserId,
}

impl AdminAuthority {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
}

#[async_trait::async_trait]
impl AdminAccess for AdminAuthority {
    async fn admin(&self) -> Result<AdminAuthority, ApiError> {
        Ok(self.clone())
    }
}
