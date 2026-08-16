use crate::endpoint::api_error::ApiError;
use crate::endpoint::json_act_media::JsonActMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::act::summaries::ActSummaries;
use crate::model::project::stage::id::StageId;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, stage_position) = path.into_inner();
    let stage_id = StageId::new(ProjectId::new(project_id), stage_position);
    let mut media = JsonActMedia::default();
    ActSummaries::new(state.pool.clone(), stage_id)
        .print(&mut media)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}
