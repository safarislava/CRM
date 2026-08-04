use crate::endpoint::api_error::ApiError;
use crate::model::user::admin::users_list::UsersList;
use crate::model::project::contract::json::Json;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn get(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let data = UsersList::new(state.pool.clone())
        .json()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
