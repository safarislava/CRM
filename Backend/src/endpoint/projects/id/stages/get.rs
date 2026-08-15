use crate::endpoint::api_error::ApiError;
use crate::endpoint::json_stage_media::JsonStageMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::project::ProjectId;
use crate::model::project::stage_summaries::StageSummaries;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let mut media = JsonStageMedia::default();
    StageSummaries::new(state.pool.clone(), ProjectId::new(path.into_inner()))
        .print(&mut media)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}
