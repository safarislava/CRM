use crate::endpoint::api_error::ApiError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::invalidating_task::InvalidatingStageTask;
use crate::model::project::stage::reordering::StageReordering;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReorderSubStageDto {
    to: i32,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, i32)>,
    body: Json<ReorderSubStageDto>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, parent_position, position) = path.into_inner();
    let project_id_obj = ProjectId::new(project_id);
    InvalidatingStageTask::new(
        StageReordering::sub(
            state.pool.clone(),
            ProjectId::new(project_id),
            parent_position,
            position,
            body.to,
        ),
        state.stage_cache.clone(),
        project_id_obj,
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
