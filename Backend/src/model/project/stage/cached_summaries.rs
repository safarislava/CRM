use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::contract::stage_media::StageMedia;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::cache_key::StageCacheKey;
use crate::model::project::stage::collecting_media::{CollectingStageMedia, StageSummaryItem};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::cache::memory_cache::MemoryCache;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use uuid::Uuid;

    struct CountedStageOrigin {
        calls: Arc<AtomicUsize>,
        project_id: ProjectId,
    }

    #[async_trait]
    impl Printer<CollectingStageMedia> for CountedStageOrigin {
        async fn print(&self, media: &mut CollectingStageMedia) -> Result<(), BoxError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            media.add_stage(
                self.project_id.id(),
                0,
                1,
                "Design Phase",
                None,
                false,
                true,
                None,
                false,
                None,
                false,
                false,
            );
            Ok(())
        }
    }

    #[actix_web::test]
    async fn fetches_stage_summaries_from_origin_on_miss_and_serves_from_cache_on_hit() {
        let cache = MemoryCache::new();
        let calls = Arc::new(AtomicUsize::new(0));
        let project_id = ProjectId::new(Uuid::new_v4());

        let origin = CountedStageOrigin {
            calls: calls.clone(),
            project_id,
        };

        let cached = CachedStageSummaries::new(origin, cache.clone(), project_id);

        // First print -> Miss (queries origin)
        let mut dest1 = CollectingStageMedia::new();
        cached.print(&mut dest1).await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(dest1.items().len(), 1);

        // Second print -> Hit (bypasses origin)
        let mut dest2 = CollectingStageMedia::new();
        cached.print(&mut dest2).await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(dest2.items().len(), 1);
    }
}
