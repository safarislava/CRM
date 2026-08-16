use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::project::ProjectId;
use crate::model::project::stage::gip::logged_gip_confirmation::LoggedGipConfirmation;
use crate::model::project::stage::stage_id::StageId;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    confirmed: bool,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32)>,
    body: Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, position) = path.into_inner();
    let stage_id = StageId::new(ProjectId::new(project_id), position);
    AuditedTask::new(
        user.clone(),
        AuditAction::GipConfirm {
            confirmed: body.confirmed,
        },
        format!("{project_id}:{position}"),
        LoggedGipConfirmation::new(state.pool.clone(), stage_id, user, body.confirmed),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
