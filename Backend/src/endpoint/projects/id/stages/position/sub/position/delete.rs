use crate::endpoint::api_error::ApiError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_task::InvalidatingStageTask;
use crate::model::project::stage::removal::StageRemoval;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, parent_position, position) = path.into_inner();
    let project_id_obj = ProjectId::new(project_id);
    InvalidatingStageTask::new(
        StageRemoval::new(
            state.pool.clone(),
            StageId::new_substage(ProjectId::new(project_id), parent_position, position),
        ),
        state.stage_cache.clone(),
        project_id_obj,
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
