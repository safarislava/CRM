use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Task: Send + Sync {
    type Output: Send;

    async fn perform(&self) -> Result<Self::Output, BoxError>;
}
