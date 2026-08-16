use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::appending::StageAppending;
use crate::model::project::stage::invalidating_by_project_id::InvalidatingByProjectId;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateStageDto {
    title: String,
}

pub async fn create(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<ProjectId>,
    body: Json<CreateStageDto>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let project_id = path.into_inner();
    let title = body.title.clone();
    AuditedTask::new(
        user,
        AuditAction::StageCreate,
        format!("{project_id}:{title}"),
        InvalidatingByProjectId::new(
            StageAppending::new(state.pool.clone(), project_id, title),
            state.stage_cache.clone(),
            project_id,
        ),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Created().finish())
}
