use crate::endpoint::api_error::ApiError;
use crate::endpoint::json_attachment_media::JsonAttachmentMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::attachment_summaries::AttachmentSummaries;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, stage_position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), stage_position);
    let mut media = JsonAttachmentMedia::default();
    AttachmentSummaries::new(state.pool.clone(), stage)
        .print(&mut media)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}