use crate::endpoint::api_error::ApiError;
use crate::model::user::admin::authority::AdminAuthority;

#[async_trait::async_trait]
pub trait AdminAccess {
    async fn admin(&self) -> Result<AdminAuthority, ApiError>;
}
