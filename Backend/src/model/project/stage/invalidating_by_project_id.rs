use crate::model::cache::contract::cache::Cache;
use crate::model::cache::invalidating_task::InvalidatingTask;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::cache_key::StageCacheKey;
use crate::model::project::stage::collecting_media::StageSummaryItem;
use async_trait::async_trait;

pub struct InvalidatingByProjectId<T, C> {
    task: InvalidatingTask<T, C, StageCacheKey, Vec<StageSummaryItem>>,
}

impl<T, C> InvalidatingByProjectId<T, C> {
    pub fn new(origin: T, cache: C, project_id: ProjectId) -> Self {
        Self {
            task: InvalidatingTask::single(origin, cache, StageCacheKey::ByProjectId(project_id)),
        }
    }
}

#[async_trait]
impl<T, C> Task for InvalidatingByProjectId<T, C>
where
    T: Task<Output = ()> + Send + Sync,
    C: Cache<StageCacheKey, Vec<StageSummaryItem>> + Send + Sync,
{
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        self.task.perform().await
    }
}
