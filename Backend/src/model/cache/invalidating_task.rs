use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use async_trait::async_trait;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct InvalidatingTask<T, C, K, V> {
    origin: T,
    cache: C,
    keys: Vec<K>,
    _phantom: PhantomData<V>,
}

impl<T, C, K, V> InvalidatingTask<T, C, K, V> {
    pub fn new(origin: T, cache: C, keys: Vec<K>) -> Self {
        Self {
            origin,
            cache,
            keys,
            _phantom: PhantomData,
        }
    }

    pub fn single(origin: T, cache: C, key: K) -> Self {
        Self::new(origin, cache, vec![key])
    }
}

#[async_trait]
impl<T, C, K, V> Task for InvalidatingTask<T, C, K, V>
where
    T: Task + Send + Sync,
    C: Cache<K, V> + Send + Sync,
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    type Output = T::Output;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let output = self.origin.perform().await?;
        for key in &self.keys {
            self.cache.evict(key).await?;
        }
        Ok(output)
    }
}
