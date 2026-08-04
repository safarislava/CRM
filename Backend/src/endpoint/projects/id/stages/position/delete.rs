use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::stage_removal::StageRemoval;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32)>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), position);
    AuditedTask::new(
        user,
        AuditAction::StageDelete,
        format!("{project_id}:{position}"),
        StageRemoval::new(state.pool.clone(), stage),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}

