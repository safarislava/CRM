use crate::endpoint::api_error::ApiError;
use crate::model::user::contract::admin_access::AdminAccess;
use crate::model::user::user::User;

#[derive(Clone, Debug)]
pub struct Admin {
    user: User,
}

impl Admin {
    pub fn new(user: User) -> Self {
        Self { user }
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}

#[async_trait::async_trait]
impl AdminAccess for Admin {
    async fn admin(&self) -> Result<Admin, ApiError> {
        Ok(self.clone())
    }
}
