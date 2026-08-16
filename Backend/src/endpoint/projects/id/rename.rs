use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::invalidating_by_project_id::InvalidatingByProjectId;
use crate::model::project::rename::ProjectRename;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RenameProjectDto {
    title: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<Uuid>,
    body: Json<RenameProjectDto>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let title = body.title.trim().to_string();
    if title.is_empty() {
        return Err(ApiError::BadRequest("Title cannot be empty".to_string()));
    }
    let raw_project_id = path.into_inner();
    let project_id = ProjectId::new(raw_project_id);
    AuditedTask::new(
        user,
        AuditAction::ProjectRename {
            new_title: title.clone(),
        },
        raw_project_id,
        InvalidatingByProjectId::new(
            ProjectRename::new(state.pool.clone(), project_id, title),
            state.project_cache.clone(),
            project_id,
        ),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
