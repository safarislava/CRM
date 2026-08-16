use crate::model::contract::box_error::BoxError;
use async_trait::async_trait;

#[async_trait]
pub trait Cache<K, V>: Send + Sync {
    async fn value(&self, key: &K) -> Result<Option<V>, BoxError>;
    async fn save(&self, key: K, value: V) -> Result<(), BoxError>;
    async fn evict(&self, key: &K) -> Result<(), BoxError>;
}
