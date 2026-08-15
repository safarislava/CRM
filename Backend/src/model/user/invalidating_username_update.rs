use crate::model::cache::contract::cache::Cache;
use crate::model::cache::invalidating_task::InvalidatingTask;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::user::user::UserId;
use crate::model::user::user_cache_key::UserCacheKey;
use async_trait::async_trait;

pub struct InvalidatingUsernameUpdate<T, C> {
    task: InvalidatingTask<T, C, UserCacheKey, UserId>,
}

impl<T, C> InvalidatingUsernameUpdate<T, C> {
    pub fn new(
        origin: T,
        cache: C,
        old_username: impl Into<String>,
        new_username: impl Into<String>,
    ) -> Self {
        let old = old_username.into();
        let new = new_username.into();
        Self {
            task: InvalidatingTask::new(
                origin,
                cache,
                vec![UserCacheKey::ByUsername(old), UserCacheKey::ByUsername(new)],
            ),
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
        self.task.perform().await
    }
}
