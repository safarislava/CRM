use crate::model::contract::box_error::BoxError;

#[async_trait::async_trait]
pub trait Scheduled: Send + Sync {
    async fn run(&self) -> Result<(), BoxError>;
}
