use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::attachment::AttachmentId;
use crate::model::project::invalidating_stage_task::InvalidatingStageTask;
use crate::model::project::logged_attachment_removal::LoggedAttachmentRemoval;
use crate::model::project::project::ProjectId;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, _position, act_id) = path.into_inner();
    let project_id_obj = ProjectId::new(project_id);
    AuditedTask::new(
        user.clone(),
        AuditAction::ActDelete,
        act_id,
        InvalidatingStageTask::new(
            LoggedAttachmentRemoval::new(
                state.pool.clone(),
                state.storage.clone(),
                AttachmentId::new(act_id),
                user,
            ),
            state.stage_cache.clone(),
            project_id_obj,
        ),
    )
    .perform()
    .await
    .map_err(|_| ApiError::NotFound("Act not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
