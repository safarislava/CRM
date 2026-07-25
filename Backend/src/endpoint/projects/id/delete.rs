use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::project::project::Project;
use crate::model::task::audit_action::AuditAction;
use crate::model::task::audited_state_task::AuditedStateTask;
use crate::model::task::contract::task::Task;
use crate::model::task::project::project_removal::ProjectRemoval;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let project_id = path.into_inner();
    AuditedStateTask::new(
        user,
        AuditAction::ProjectDelete,
        project_id,
        ProjectRemoval::new(state.pool.clone(), Project::new(project_id)),
    )
    .done()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
