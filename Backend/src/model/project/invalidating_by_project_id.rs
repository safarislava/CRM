use crate::model::cache::contract::cache::Cache;
use crate::model::cache::invalidating_task::InvalidatingTask;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::cache_key::ProjectCacheKey;
use crate::model::project::cached_summaries::ProjectSummaryItem;
use crate::model::project::id::ProjectId;
use async_trait::async_trait;

pub struct InvalidatingByProjectId<T, C> {
    task: InvalidatingTask<T, C, ProjectCacheKey, Vec<ProjectSummaryItem>>,
}

impl<T, C> InvalidatingByProjectId<T, C> {
    pub fn new(origin: T, cache: C, project_id: ProjectId) -> Self {
        Self {
            task: InvalidatingTask::new(
                origin,
                cache,
                vec![
                    ProjectCacheKey::AllSummaries,
                    ProjectCacheKey::ByProjectId(project_id),
                ],
            ),
        }
    }
}

#[async_trait]
impl<T, C> Task for InvalidatingByProjectId<T, C>
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
    async fn evicts_all_summaries_and_project_id_from_cache() {
        let cache = MemoryCache::new();
        let project_id = ProjectId::new(Uuid::new_v4());

        cache
            .save(ProjectCacheKey::AllSummaries, vec![])
            .await
            .unwrap();
        cache
            .save(ProjectCacheKey::ByProjectId(project_id), vec![])
            .await
            .unwrap();

        let flag = Arc::new(AtomicBool::new(false));
        let decorator =
            InvalidatingByProjectId::new(FlagTask(flag.clone()), cache.clone(), project_id);

        decorator.perform().await.unwrap();

        assert!(flag.load(Ordering::SeqCst));
        assert!(
            cache
                .value(&ProjectCacheKey::AllSummaries)
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            cache
                .value(&ProjectCacheKey::ByProjectId(project_id))
                .await
                .unwrap()
                .is_none()
        );
    }
}
