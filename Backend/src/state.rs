use crate::mail::Mailer;
use crate::model::cache::memory_cache::MemoryCache;
use crate::model::project::cached_project_summaries::ProjectSummaryItem;
use crate::model::project::collecting_stage_media::StageSummaryItem;
use crate::model::project::project_cache_key::ProjectCacheKey;
use crate::model::project::stage_cache_key::StageCacheKey;
use crate::model::user::user::UserId;
use crate::model::user::user_cache_key::UserCacheKey;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub pool: Arc<PgPool>,
    pub storage: Arc<Storage>,
    pub mailer: Arc<Mailer>,
    pub project_cache: MemoryCache<ProjectCacheKey, Vec<ProjectSummaryItem>>,
    pub stage_cache: MemoryCache<StageCacheKey, Vec<StageSummaryItem>>,
    pub user_cache: MemoryCache<UserCacheKey, UserId>,
}
