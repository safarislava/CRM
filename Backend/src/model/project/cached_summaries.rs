use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::contract::project_media::ProjectMedia;
use crate::model::project::cache_key::ProjectCacheKey;
use crate::model::project::collecting_media::CollectingProjectMedia;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ProjectSummaryItem {
    pub id: Uuid,
    pub title: String,
    pub updated_at: DateTime<Utc>,
}

pub struct CachedProjectSummaries<T, C> {
    origin: T,
    cache: C,
}

impl<T, C> CachedProjectSummaries<T, C> {
    pub fn new(origin: T, cache: C) -> Self {
        Self { origin, cache }
    }
}

#[async_trait]
impl<T, C, M> Printer<M> for CachedProjectSummaries<T, C>
where
    T: Printer<CollectingProjectMedia> + Send + Sync,
    C: Cache<ProjectCacheKey, Vec<ProjectSummaryItem>> + Send + Sync,
    M: ProjectMedia + Send + Sync,
{
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        let key = ProjectCacheKey::AllSummaries;
        let items = match self.cache.value(&key).await? {
            Some(cached) => cached,
            None => {
                let mut collector = CollectingProjectMedia::new();
                self.origin.print(&mut collector).await?;
                let items = collector.items();
                self.cache.save(key, items.clone()).await?;
                items
            }
        };
        for item in &items {
            media.add_project(item.id, &item.title, item.updated_at);
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

    struct CountedOrigin {
        calls: Arc<AtomicUsize>,
        items: Vec<(Uuid, String, DateTime<Utc>)>,
    }

    #[async_trait]
    impl Printer<CollectingProjectMedia> for CountedOrigin {
        async fn print(&self, media: &mut CollectingProjectMedia) -> Result<(), BoxError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            for (id, title, updated_at) in &self.items {
                media.add_project(*id, title, *updated_at);
            }
            Ok(())
        }
    }

    #[actix_web::test]
    async fn fetches_from_origin_on_cache_miss_and_serves_from_cache_on_hit() {
        let cache = MemoryCache::new();
        let calls = Arc::new(AtomicUsize::new(0));
        let p_id = Uuid::new_v4();
        let now = Utc::now();

        let origin = CountedOrigin {
            calls: calls.clone(),
            items: vec![(p_id, "Project X".to_string(), now)],
        };

        let cached_summaries = CachedProjectSummaries::new(origin, cache.clone());

        // First print -> Cache Miss (calls origin)
        let mut dest1 = CollectingProjectMedia::new();
        cached_summaries.print(&mut dest1).await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(dest1.items().len(), 1);

        // Second print -> Cache Hit (skips origin)
        let mut dest2 = CollectingProjectMedia::new();
        cached_summaries.print(&mut dest2).await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(dest2.items().len(), 1);
    }
}
