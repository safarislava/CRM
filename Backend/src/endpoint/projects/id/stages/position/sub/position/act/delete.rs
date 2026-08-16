use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::attachment::id::AttachmentId;
use crate::model::project::stage::attachment::logged_removal::LoggedAttachmentRemoval;
use crate::model::project::stage::invalidating_task::InvalidatingStageTask;
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
    let (project_id, _, _, act_id) = path.into_inner();
    let project_id_obj = ProjectId::new(project_id);
    InvalidatingStageTask::new(
        LoggedAttachmentRemoval::new(
            state.pool.clone(),
            state.storage.clone(),
            AttachmentId::new(act_id),
            user,
        ),
        state.stage_cache.clone(),
        project_id_obj,
    )
    .perform()
    .await
    .map_err(|_| ApiError::NotFound("Act not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
