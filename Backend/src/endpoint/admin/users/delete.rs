use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::admin::user_deletion::UserDeletion;
use crate::model::task::contract::task::Task;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let admin = request
        .admin()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;

    let target_user_id = path.into_inner();
    UserDeletion::new(state.pool.clone(), admin, target_user_id)
        .done()
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    Ok(HttpResponse::NoContent().finish())
}
