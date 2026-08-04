use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Value<T> {
    async fn value(&self) -> Result<T, BoxError>;
}
