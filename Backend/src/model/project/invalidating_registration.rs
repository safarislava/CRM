use crate::model::cache::contract::cache::Cache;
use crate::model::cache::invalidating_task::InvalidatingTask;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::cache_key::ProjectCacheKey;
use crate::model::project::cached_summaries::ProjectSummaryItem;
use async_trait::async_trait;

pub struct InvalidatingProjectRegistration<T, C> {
    task: InvalidatingTask<T, C, ProjectCacheKey, Vec<ProjectSummaryItem>>,
}

impl<T, C> InvalidatingProjectRegistration<T, C> {
    pub fn new(origin: T, cache: C) -> Self {
        Self {
            task: InvalidatingTask::single(origin, cache, ProjectCacheKey::AllSummaries),
        }
    }
}

#[async_trait]
impl<T, C> Task for InvalidatingProjectRegistration<T, C>
where
    T: Task<Output = ()> + Send + Sync,
    C: Cache<ProjectCacheKey, Vec<ProjectSummaryItem>> + Send + Sync,
{
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        self.task.perform().await
    }
}
