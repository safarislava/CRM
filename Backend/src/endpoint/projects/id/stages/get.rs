use super::media::JsonStageMedia;
use crate::endpoint::api_error::ApiError;
use crate::model::contract::printer::Printer;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::cached_summaries::CachedStageSummaries;
use crate::model::project::stage::summaries::StageSummaries;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let mut media = JsonStageMedia::default();
    let project_id = ProjectId::new(path.into_inner());
    CachedStageSummaries::new(
        StageSummaries::new(state.pool.clone(), project_id),
        state.stage_cache.clone(),
        project_id,
    )
    .print(&mut media)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}
