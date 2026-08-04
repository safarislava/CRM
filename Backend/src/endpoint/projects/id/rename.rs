use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::project::project::Project;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::project_rename::ProjectRename;
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
    let project_id = path.into_inner();
    AuditedTask::new(
        user,
        AuditAction::ProjectRename {
            new_title: title.clone(),
        },
        project_id,
        ProjectRename::new(state.pool.clone(), Project::new(project_id), title),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
