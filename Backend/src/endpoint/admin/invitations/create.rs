use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::user::invite_creation::InviteCreation;
use crate::model::user::contract::invite::Invite;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin = request
        .admin()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;

    let invite = AuditedTask::new(
        admin.user().clone(),
        AuditAction::InviteCreate,
        "invite",
        InviteCreation::new(state.pool.clone(), admin.user().clone()),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(HttpResponse::Created().json(serde_json::json!({ "token": invite.token() })))
}
