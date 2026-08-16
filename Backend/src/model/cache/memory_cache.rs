use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

pub struct MemoryCache<K, V> {
    items: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> Clone for MemoryCache<K, V> {
    fn clone(&self) -> Self {
        Self {
            items: Arc::clone(&self.items),
        }
    }
}

impl<K, V> MemoryCache<K, V> {
    pub fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[allow(dead_code)]
    pub fn with_items(items: Arc<RwLock<HashMap<K, V>>>) -> Self {
        Self { items }
    }
}

impl<K, V> Default for MemoryCache<K, V> {
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
        let guard = self
            .items
            .read()
            .map_err(|e| format!("MemoryCache read lock poisoned: {}", e))?;
        Ok(guard.get(key).cloned())
    }

    async fn save(&self, key: K, value: V) -> Result<(), BoxError> {
        let mut guard = self
            .items
            .write()
            .map_err(|e| format!("MemoryCache write lock poisoned: {}", e))?;
        guard.insert(key, value);
        Ok(())
    }

    async fn evict(&self, key: &K) -> Result<(), BoxError> {
        let mut guard = self
            .items
            .write()
            .map_err(|e| format!("MemoryCache write lock poisoned: {}", e))?;
        guard.remove(key);
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
