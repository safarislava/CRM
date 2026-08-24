use crate::model::contract::box_error::BoxError;

#[async_trait::async_trait]
pub trait Schedule: Send + Sync {
    async fn run(&self) -> Result<(), BoxError>;
}
