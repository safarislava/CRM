use crate::endpoint::api_error::ApiError;
use crate::endpoint::json_comment_media::JsonCommentMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::comment_summaries::CommentSummaries;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
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
    let stage = Stage::new(Project::new(project_id), stage_position);
    let mut media = JsonCommentMedia::default();
    CommentSummaries::new(state.pool.clone(), stage, query.into_inner().before)
        .print(&mut media)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}