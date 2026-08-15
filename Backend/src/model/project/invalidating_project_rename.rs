use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::cache_key::ProjectCacheKey;
use crate::model::project::cached_project_summaries::ProjectSummaryItem;
use async_trait::async_trait;
use uuid::Uuid;

pub struct InvalidatingProjectRename<T, C> {
    origin: T,
    cache: C,
    project_id: Uuid,
}

impl<T, C> InvalidatingProjectRename<T, C> {
    pub fn new(origin: T, cache: C, project_id: Uuid) -> Self {
        Self {
            origin,
            cache,
            project_id,
        }
    }
}

#[async_trait]
impl<T, C> Task for InvalidatingProjectRename<T, C>
where
    T: Task<Output = ()> + Send + Sync,
    C: Cache<ProjectCacheKey, Vec<ProjectSummaryItem>> + Send + Sync,
{
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        self.origin.perform().await?;
        let _ = self.cache.evict(&ProjectCacheKey::AllSummaries).await;
        let _ = self.cache.evict(&ProjectCacheKey::ById(self.project_id)).await;
        Ok(())
    }
}
