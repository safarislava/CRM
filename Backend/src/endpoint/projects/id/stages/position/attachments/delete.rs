use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::attachment::AttachmentId;
use crate::model::project::logged_attachment_removal::LoggedAttachmentRemoval;
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
    let (_, _, attachment_id) = path.into_inner();
    AuditedTask::new(
        user.clone(),
        AuditAction::AttachmentDelete {
            filename: attachment_id.to_string(),
        },
        attachment_id,
        LoggedAttachmentRemoval::new(
            state.pool.clone(),
            state.storage.clone(),
            AttachmentId::new(attachment_id),
            user,
        ),
    )
    .perform()
    .await
    .map_err(|_| ApiError::NotFound("Attachment not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
