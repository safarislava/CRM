use crate::model::contract::box_error::BoxError;

#[async_trait::async_trait]
pub trait Cookie: Send + Sync {
    async fn value(&self) -> Result<actix_web::cookie::Cookie<'static>, BoxError>;
}