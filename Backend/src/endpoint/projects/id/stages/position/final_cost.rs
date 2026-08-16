use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::cost::r#final::logged_update::LoggedFinalCostUpdate;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_task::InvalidatingStageTask;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateCostDto {
    cost: Option<i32>,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32)>,
    body: Json<UpdateCostDto>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, position) = path.into_inner();
    let project_id_obj = ProjectId::new(project_id);
    let stage_id = StageId::new(project_id_obj, position);
    AuditedTask::new(
        user.clone(),
        AuditAction::FinalCostUpdate {
            new_cost: body.cost,
        },
        format!("{project_id}:{position}"),
        InvalidatingStageTask::new(
            LoggedFinalCostUpdate::new(state.pool.clone(), stage_id, user, body.cost),
            state.stage_cache.clone(),
            project_id_obj,
        ),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
