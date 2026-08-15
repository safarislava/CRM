use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::credential::contract::username::Username;
use crate::model::user::cache_key::UserCacheKey;
use crate::model::user::contract::username_search::UsernameSearch;
use crate::model::user::user::User;
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
    C: Cache<UserCacheKey, User> + Send + Sync,
{
    async fn found(&self, username: impl Username) -> Result<Option<User>, BoxError> {
        let name = username.value()?;
        let key = UserCacheKey::ByUsername(name);
        if let Some(user) = self.cache.value(&key).await? {
            return Ok(Some(user));
        }
        let result = self.origin.found(username).await?;
        if let Some(user) = &result {
            self.cache.save(key, user.clone()).await?;
        }
        Ok(result)
    }
}
