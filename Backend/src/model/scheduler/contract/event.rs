use crate::model::contract::box_error::BoxError;

#[async_trait::async_trait]
pub trait Event: Send + Sync {
    async fn fired(&self) -> Result<(), BoxError>;
}
