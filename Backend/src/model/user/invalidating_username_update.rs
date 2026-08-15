use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::user::user::UserId;
use crate::model::user::user_cache_key::UserCacheKey;
use async_trait::async_trait;
use uuid::Uuid;

pub struct InvalidatingUsernameUpdate<T, C> {
    origin: T,
    cache: C,
    user_id: Uuid,
    old_username: String,
    new_username: String,
}

impl<T, C> InvalidatingUsernameUpdate<T, C> {
    pub fn new(
        origin: T,
        cache: C,
        user_id: Uuid,
        old_username: impl Into<String>,
        new_username: impl Into<String>,
    ) -> Self {
        Self {
            origin,
            cache,
            user_id,
            old_username: old_username.into(),
            new_username: new_username.into(),
        }
    }
}

#[async_trait]
impl<T, C> Task for InvalidatingUsernameUpdate<T, C>
where
    T: Task<Output = ()> + Send + Sync,
    C: Cache<UserCacheKey, UserId> + Send + Sync,
{
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        self.origin.perform().await?;
        for key in [
            UserCacheKey::ByUsername(self.old_username.clone()),
            UserCacheKey::ByUsername(self.new_username.clone()),
            UserCacheKey::ById(self.user_id),
        ] {
            let _ = self.cache.evict(&key).await;
        }
        Ok(())
    }
}
