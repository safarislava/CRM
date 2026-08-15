use crate::model::cache::contract::cache::Cache;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::contract::project_media::ProjectMedia;
use crate::model::project::cache_key::ProjectCacheKey;
use crate::model::project::collecting_project_media::CollectingProjectMedia;
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
