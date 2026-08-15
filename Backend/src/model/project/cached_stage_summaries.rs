use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::contract::stage_media::StageMedia;
use crate::model::project::collecting_stage_media::{CollectingStageMedia, StageSummaryItem};
use crate::model::project::project::ProjectId;
use crate::model::project::stage_cache_key::StageCacheKey;
use async_trait::async_trait;

pub struct CachedStageSummaries<T, C> {
    origin: T,
    cache: C,
    project_id: ProjectId,
}

impl<T, C> CachedStageSummaries<T, C> {
    pub fn new(origin: T, cache: C, project_id: ProjectId) -> Self {
        Self {
            origin,
            cache,
            project_id,
        }
    }
}

#[async_trait]
impl<T, C, M> Printer<M> for CachedStageSummaries<T, C>
where
    T: Printer<CollectingStageMedia> + Send + Sync,
    C: Cache<StageCacheKey, Vec<StageSummaryItem>> + Send + Sync,
    M: StageMedia + Send + Sync,
{
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        let key = StageCacheKey::ByProjectId(self.project_id);
        let items = match self.cache.value(&key).await? {
            Some(cached) => cached,
            None => {
                let mut collector = CollectingStageMedia::new();
                self.origin.print(&mut collector).await?;
                let items = collector.items();
                self.cache.save(key, items.clone()).await?;
                items
            }
        };
        for item in &items {
            media.add_stage(
                item.project_id,
                item.parent_position,
                item.position,
                &item.title,
                item.deadline,
                item.completed,
                item.gip_confirmed,
                item.advance_cost,
                item.advance_confirmed,
                item.final_cost,
                item.final_confirmed,
                item.has_act,
            );
        }
        Ok(())
    }
}
