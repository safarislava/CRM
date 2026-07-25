use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::project::project::Project;
use crate::model::task::audit_action::AuditAction;
use crate::model::task::audited_state_task::AuditedStateTask;
use crate::model::task::contract::task::Task;
use crate::model::task::project::stage_appending::StageAppending;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateStageDto {
    title: String,
}

pub async fn create(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<Uuid>,
    body: Json<CreateStageDto>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let project_id = path.into_inner();
    let title = body.title.clone();
    AuditedStateTask::new(
        user,
        AuditAction::StageCreate,
        format!("{project_id}:{title}"),
        StageAppending::new(state.pool.clone(), Project::new(project_id), title),
    )
    .done()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}

