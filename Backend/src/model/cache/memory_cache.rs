use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use async_trait::async_trait;
use moka::future::Cache as MokaCache;
use std::hash::Hash;

pub struct MemoryCache<K, V> {
    items: MokaCache<K, V>,
}

impl<K, V> Clone for MemoryCache<K, V> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
        }
    }
}

impl<K, V> MemoryCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            items: MokaCache::builder().build(),
        }
    }

    #[allow(dead_code)]
    pub fn with_capacity(max_capacity: u64) -> Self {
        Self {
            items: MokaCache::builder().max_capacity(max_capacity).build(),
        }
    }

    #[allow(dead_code)]
    pub fn with_cache(cache: MokaCache<K, V>) -> Self {
        Self { items: cache }
    }
}

impl<K, V> Default for MemoryCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<K, V> Cache<K, V> for MemoryCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    async fn value(&self, key: &K) -> Result<Option<V>, BoxError> {
        Ok(self.items.get(key).await)
    }

    async fn save(&self, key: K, value: V) -> Result<(), BoxError> {
        self.items.insert(key, value).await;
        Ok(())
    }

    async fn evict(&self, key: &K) -> Result<(), BoxError> {
        self.items.invalidate(key).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn saves_retrieves_and_evicts_cache_entries() {
        let cache: MemoryCache<String, i32> = MemoryCache::new();

        assert_eq!(cache.value(&"key1".to_string()).await.unwrap(), None);

        cache.save("key1".to_string(), 42).await.unwrap();
        assert_eq!(cache.value(&"key1".to_string()).await.unwrap(), Some(42));

        cache.evict(&"key1".to_string()).await.unwrap();
        assert_eq!(cache.value(&"key1".to_string()).await.unwrap(), None);
    }

    #[actix_web::test]
    async fn supports_concurrency_across_clones() {
        let cache: MemoryCache<String, String> = MemoryCache::new();
        let clone1 = cache.clone();
        let clone2 = cache.clone();

        clone1
            .save("k".to_string(), "v1".to_string())
            .await
            .unwrap();
        assert_eq!(
            clone2.value(&"k".to_string()).await.unwrap(),
            Some("v1".to_string())
        );
    }
}
