use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::cost::logged_advance_payment_confirmation::LoggedAdvancePaymentConfirmation;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_task::InvalidatingStageTask;
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
    InvalidatingStageTask::new(
        LoggedAdvancePaymentConfirmation::new(state.pool.clone(), stage_id, user, body.confirmed),
        state.stage_cache.clone(),
        stage_id.project_id(),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
