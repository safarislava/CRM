use crate::model::cache::contract::cache::Cache;
use crate::model::cache::invalidating_task::InvalidatingTask;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::cache_key::ProjectCacheKey;
use crate::model::project::cached_summaries::ProjectSummaryItem;
use async_trait::async_trait;

pub struct InvalidatingAllSummaries<T, C> {
    task: InvalidatingTask<T, C, ProjectCacheKey, Vec<ProjectSummaryItem>>,
}

impl<T, C> InvalidatingAllSummaries<T, C> {
    pub fn new(origin: T, cache: C) -> Self {
        Self {
            task: InvalidatingTask::single(origin, cache, ProjectCacheKey::AllSummaries),
        }
    }
}

#[async_trait]
impl<T, C> Task for InvalidatingAllSummaries<T, C>
where
    T: Task<Output = ()> + Send + Sync,
    C: Cache<ProjectCacheKey, Vec<ProjectSummaryItem>> + Send + Sync,
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
    async fn performs_origin_task_and_evicts_all_summaries_cache() {
        let cache = MemoryCache::new();
        cache
            .save(ProjectCacheKey::AllSummaries, vec![])
            .await
            .unwrap();

        let flag = Arc::new(AtomicBool::new(false));
        let decorator = InvalidatingAllSummaries::new(FlagTask(flag.clone()), cache.clone());

        decorator.perform().await.unwrap();

        assert!(flag.load(Ordering::SeqCst));
        assert!(
            cache
                .value(&ProjectCacheKey::AllSummaries)
                .await
                .unwrap()
                .is_none()
        );
    }
}
