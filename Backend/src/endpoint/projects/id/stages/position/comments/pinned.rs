use super::media::JsonCommentMedia;
use crate::endpoint::api_error::ApiError;
use crate::model::contract::printer::Printer;
use crate::model::project::stage::comment::pinned_summaries::PinnedCommentSummaries;
use crate::model::project::stage::id::StageId;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn get(state: web::Data<AppState>, stage_id: StageId) -> Result<HttpResponse, ApiError> {
    let mut media = JsonCommentMedia::default();
    PinnedCommentSummaries::new(state.pool.clone(), stage_id)
        .print(&mut media)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}
