use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::stage::comment::creation::CommentCreation;
use crate::model::project::stage::id::StageId;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Body {
    text: String,
}

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
    stage_id: StageId,
    body: web::Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let text = body.into_inner().text;
    if text.trim().is_empty() {
        return Err(ApiError::BadRequest("Text must not be empty".to_string()));
    }
    let project_id = stage_id.project_id();
    let position = stage_id.position();
    AuditedTask::new(
        user.clone(),
        AuditAction::CommentCreate { text: text.clone() },
        format!("{project_id}:{position}"),
        CommentCreation::new(state.pool.clone(), stage_id, user, text),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Created().finish())
}
