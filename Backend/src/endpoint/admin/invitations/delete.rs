use crate::endpoint::api_error::ApiError;
use crate::model::user::admin::invitation_revocation::InvitationRevocation;
use crate::model::contract::task::Task;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let token = path.into_inner();
    InvitationRevocation::new(state.pool.clone(), token)
        .perform()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(HttpResponse::NoContent().finish())
}
