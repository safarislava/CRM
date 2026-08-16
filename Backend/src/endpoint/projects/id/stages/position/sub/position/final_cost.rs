use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::contract::task::Task;
use crate::model::project::project::ProjectId;
use crate::model::project::stage::cost::logged_final_cost_update::LoggedFinalCostUpdate;
use crate::model::project::stage::invalidating_stage_task::InvalidatingStageTask;
use crate::model::project::stage::stage_id::StageId;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    cost: Option<i32>,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32, i32)>,
    body: Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, parent_position, position) = path.into_inner();
    let project_id_obj = ProjectId::new(project_id);
    InvalidatingStageTask::new(
        LoggedFinalCostUpdate::new(
            state.pool.clone(),
            StageId::new_substage(ProjectId::new(project_id), parent_position, position),
            user,
            body.cost,
        ),
        state.stage_cache.clone(),
        project_id_obj,
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
