use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::invalidating_by_project_id::InvalidatingByProjectId;
use crate::model::project::stage::reordering::StageReordering;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReorderStageDto {
    to: i32,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32)>,
    body: Json<ReorderStageDto>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, position) = path.into_inner();
    let project_id_obj = ProjectId::new(project_id);
    AuditedTask::new(
        user,
        AuditAction::StageReorder { to: body.to },
        format!("{project_id}:{position}"),
        InvalidatingByProjectId::new(
            StageReordering::new(state.pool.clone(), project_id_obj, position, body.to),
            state.stage_cache.clone(),
            project_id_obj,
        ),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
