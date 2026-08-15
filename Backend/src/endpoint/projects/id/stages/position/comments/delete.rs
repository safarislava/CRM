use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::comment::CommentId;
use crate::model::project::comment_removal::CommentRemoval;
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
    let (_, _, comment_id) = path.into_inner();
    AuditedTask::new(
        user,
        AuditAction::CommentDelete,
        comment_id,
        CommentRemoval::new(state.pool.clone(), CommentId::new(comment_id)),
    )
    .perform()
    .await
    .map_err(|_| ApiError::NotFound("Comment not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
