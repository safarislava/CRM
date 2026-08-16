use crate::endpoint::api_error::ApiError;
use crate::model::project::contract::json::Json;
use crate::model::user::admin::invitations_list::InvitationsList;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn get(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let data = InvitationsList::new(state.pool.clone())
        .json()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
