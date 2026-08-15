use crate::model::cache::contract::cache::Cache;
use crate::model::cache::invalidating_task::InvalidatingTask;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::cached_project_summaries::ProjectSummaryItem;
use crate::model::project::project::ProjectId;
use crate::model::project::project_cache_key::ProjectCacheKey;
use async_trait::async_trait;

pub struct InvalidatingProjectRemoval<T, C> {
    task: InvalidatingTask<T, C, ProjectCacheKey, Vec<ProjectSummaryItem>>,
}

impl<T, C> InvalidatingProjectRemoval<T, C> {
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
impl<T, C> Task for InvalidatingProjectRemoval<T, C>
where
    T: Task<Output = ()> + Send + Sync,
    C: Cache<ProjectCacheKey, Vec<ProjectSummaryItem>> + Send + Sync,
{
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        self.task.perform().await
    }
}
