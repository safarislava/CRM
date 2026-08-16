use crate::endpoint::api_error::ApiError;
use crate::model::contract::task::Task;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_by_project_id::InvalidatingByProjectId;
use crate::model::project::stage::removal::StageRemoval;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn delete(
    state: web::Data<AppState>,
    stage_id: StageId,
) -> Result<HttpResponse, ApiError> {
    InvalidatingByProjectId::new(
        StageRemoval::new(state.pool.clone(), stage_id),
        state.stage_cache.clone(),
        stage_id.project_id(),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
