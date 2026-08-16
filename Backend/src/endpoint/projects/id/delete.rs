use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::invalidating_by_project_id::InvalidatingByProjectId;
use crate::model::project::removal::ProjectRemoval;
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
    let raw_project_id = path.into_inner();
    let project_id = ProjectId::new(raw_project_id);
    AuditedTask::new(
        user,
        AuditAction::ProjectDelete,
        raw_project_id,
        InvalidatingByProjectId::new(
            ProjectRemoval::new(state.pool.clone(), project_id),
            state.project_cache.clone(),
            project_id,
        ),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
