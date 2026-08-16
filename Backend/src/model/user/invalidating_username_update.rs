use crate::model::cache::contract::cache::Cache;
use crate::model::cache::invalidating_task::InvalidatingTask;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::user::cache_key::UserCacheKey;
use crate::model::user::id::UserId;
use async_trait::async_trait;

pub struct InvalidatingUsernameUpdate<T, C> {
    task: InvalidatingTask<T, C, UserCacheKey, UserId>,
}

impl<T, C> InvalidatingUsernameUpdate<T, C> {
    pub fn new(
        origin: T,
        cache: C,
        user_id: UserId,
        old_username: impl Into<String>,
        new_username: impl Into<String>,
    ) -> Self {
        let old = old_username.into();
        let new = new_username.into();
        Self {
            task: InvalidatingTask::new(
                origin,
                cache,
                vec![
                    UserCacheKey::ByUsername(old),
                    UserCacheKey::ByUsername(new),
                    UserCacheKey::ById(user_id),
                ],
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::cache::memory_cache::MemoryCache;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use uuid::Uuid;

    struct FlagTask(Arc<AtomicBool>);

    #[async_trait]
    impl Task for FlagTask {
        type Output = ();
        async fn perform(&self) -> Result<(), BoxError> {
            self.0.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    #[actix_web::test]
    async fn evicts_old_and_new_usernames_and_id_from_user_cache() {
        let cache = MemoryCache::new();
        let user_id = UserId::new(Uuid::new_v4());

        cache
            .save(UserCacheKey::ByUsername("old_name".to_string()), user_id)
            .await
            .unwrap();
        cache
            .save(UserCacheKey::ByUsername("new_name".to_string()), user_id)
            .await
            .unwrap();
        cache
            .save(UserCacheKey::ById(user_id), user_id)
            .await
            .unwrap();

        let flag = Arc::new(AtomicBool::new(false));
        let decorator = InvalidatingUsernameUpdate::new(
            FlagTask(flag.clone()),
            cache.clone(),
            user_id,
            "old_name",
            "new_name",
        );

        decorator.perform().await.unwrap();

        assert!(flag.load(Ordering::SeqCst));
        assert!(
            cache
                .value(&UserCacheKey::ByUsername("old_name".to_string()))
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            cache
                .value(&UserCacheKey::ByUsername("new_name".to_string()))
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            cache
                .value(&UserCacheKey::ById(user_id))
                .await
                .unwrap()
                .is_none()
        );
    }
}
