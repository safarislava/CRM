use crate::endpoint::api_error::ApiError;
use crate::model::project::contract::json::Json;
use crate::model::project::project::ProjectId;
use crate::model::project::stage::detailed_stage::DetailedStage;
use crate::model::project::stage::stage_id::StageId;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, position) = path.into_inner();
    let stage_id = DetailedStage::new(
        state.pool.clone(),
        StageId::new(ProjectId::new(project_id), position),
    );
    let data = stage_id
        .json()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(data))
}
