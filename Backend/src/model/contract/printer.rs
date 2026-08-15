use crate::model::contract::box_error::BoxError;

#[async_trait::async_trait]
pub trait Printer<M>: Send + Sync {
    async fn print(&self, media: &mut M) -> Result<(), BoxError>;
}
