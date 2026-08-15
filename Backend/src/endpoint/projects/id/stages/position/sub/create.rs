use crate::endpoint::api_error::ApiError;
use crate::model::contract::task::Task;
use crate::model::project::invalidating_stage_task::InvalidatingStageTask;
use crate::model::project::project::ProjectId;
use crate::model::project::stage_appending::StageAppending;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    title: String,
}

pub async fn post(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, parent_position) = path.into_inner();
    let project_id_obj = ProjectId::new(project_id);
    InvalidatingStageTask::new(
        StageAppending::sub(
            state.pool.clone(),
            ProjectId::new(project_id),
            parent_position,
            body.title.clone(),
        ),
        state.stage_cache.clone(),
        project_id_obj,
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
