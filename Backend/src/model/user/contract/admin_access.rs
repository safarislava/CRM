use crate::endpoint::api_error::ApiError;
use crate::model::user::admin::Admin;

#[async_trait::async_trait]
pub trait AdminAccess {
    async fn admin(&self) -> Result<Admin, ApiError>;
}
