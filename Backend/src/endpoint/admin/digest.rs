use crate::endpoint::api_error::ApiError;
use crate::model::task::contract::task::Task;
use crate::model::task::notification::deadline_digest_notification::DeadlineDigestNotification;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn post(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    DeadlineDigestNotification::new(state.pool.clone(), state.mailer.clone())
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().finish())
}
