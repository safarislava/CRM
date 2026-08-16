use crate::endpoint::api_error::ApiError;
use crate::endpoint::json_comment_media::JsonCommentMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::comment::summaries::CommentSummaries;
use crate::model::project::stage::id::StageId;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Query {
    before: Option<Uuid>,
}

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, stage_position) = path.into_inner();
    let stage_id = StageId::new(ProjectId::new(project_id), stage_position);
    let mut media = JsonCommentMedia::default();
    CommentSummaries::new(state.pool.clone(), stage_id, query.into_inner().before)
        .print(&mut media)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}
