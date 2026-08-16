use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::contract::task::Task;
use crate::model::project::stage::attachment::attachment::AttachmentId;
use crate::model::project::stage::attachment::logged_attachment_removal::LoggedAttachmentRemoval;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32, i32, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (_, _, _, attachment_id) = path.into_inner();
    LoggedAttachmentRemoval::new(
        state.pool.clone(),
        state.storage.clone(),
        AttachmentId::new(attachment_id),
        user,
    )
    .perform()
    .await
    .map_err(|_| ApiError::NotFound("Attachment not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
