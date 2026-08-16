use crate::endpoint::api_error::ApiError;
use crate::model::project::contract::json::Json;
use crate::model::project::stage::detailed::DetailedStage;
use crate::model::project::stage::id::StageId;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn get(state: web::Data<AppState>, stage_id: StageId) -> Result<HttpResponse, ApiError> {
    let data = DetailedStage::new(state.pool.clone(), stage_id)
        .json()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(data))
}
