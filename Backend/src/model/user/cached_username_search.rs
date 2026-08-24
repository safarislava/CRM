use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::credential::contract::username::Username;
use crate::model::user::cache_key::UserCacheKey;
use crate::model::user::contract::username_search::UsernameSearch;
use crate::model::user::id::UserId;
use async_trait::async_trait;

pub struct CachedUsernameSearch<T, C> {
    origin: T,
    cache: C,
}

impl<T, C> CachedUsernameSearch<T, C> {
    pub fn new(origin: T, cache: C) -> Self {
        Self { origin, cache }
    }
}

#[async_trait]
impl<T, C> UsernameSearch for CachedUsernameSearch<T, C>
where
    T: UsernameSearch + Send + Sync,
    C: Cache<UserCacheKey, UserId> + Send + Sync,
{
    async fn found(&self, username: impl Username) -> Result<Option<UserId>, BoxError> {
        let name = username.value().await?;
        let key = UserCacheKey::ByUsername(name);
        if let Some(user) = self.cache.value(&key).await? {
            return Ok(Some(user));
        }
        let result = self.origin.found(username).await?;
        if let Some(user) = &result {
            self.cache.save(key, *user).await?;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::cache::memory_cache::MemoryCache;
    use crate::model::credential::raw_username::RawUsername;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use uuid::Uuid;

    struct CountedSearch {
        calls: Arc<AtomicUsize>,
        found_id: UserId,
    }

    #[async_trait]
    impl UsernameSearch for CountedSearch {
        async fn found(&self, _username: impl Username) -> Result<Option<UserId>, BoxError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(Some(self.found_id))
        }
    }

    #[actix_web::test]
    async fn searches_origin_on_miss_and_serves_from_cache_on_hit() {
        let cache = MemoryCache::new();
        let calls = Arc::new(AtomicUsize::new(0));
        let user_id = UserId::new(Uuid::new_v4());

        let origin = CountedSearch {
            calls: calls.clone(),
            found_id: user_id,
        };
        let search = CachedUsernameSearch::new(origin, cache);

        let uname1 = RawUsername::new("john_doe".to_string());
        let uname2 = RawUsername::new("john_doe".to_string());

        // Miss
        let res1 = search.found(uname1).await.unwrap();
        assert_eq!(res1, Some(user_id));
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        // Hit
        let res2 = search.found(uname2).await.unwrap();
        assert_eq!(res2, Some(user_id));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
