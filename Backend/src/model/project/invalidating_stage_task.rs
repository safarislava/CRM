use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::collecting_stage_media::StageSummaryItem;
use crate::model::project::project::Project;
use crate::model::project::stage_cache_key::StageCacheKey;
use async_trait::async_trait;

pub struct InvalidatingStageTask<T, C> {
    origin: T,
    cache: C,
    project: Project,
}

impl<T, C> InvalidatingStageTask<T, C> {
    pub fn new(origin: T, cache: C, project: Project) -> Self {
        Self {
            origin,
            cache,
            project,
        }
    }
}

#[async_trait]
impl<T, C> Task for InvalidatingStageTask<T, C>
where
    T: Task<Output = ()> + Send + Sync,
    C: Cache<StageCacheKey, Vec<StageSummaryItem>> + Send + Sync,
{
    type Output = ();

    async fn perform(&self) -> Result<(), BoxError> {
        self.origin.perform().await?;
        let _ = self
            .cache
            .evict(&StageCacheKey::ByProject(self.project))
            .await;
        Ok(())
    }
}
