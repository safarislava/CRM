use crate::endpoint::api_error::ApiError;
use crate::model::project::contract::json::Json;
use crate::model::user::admin::system_logs::SystemLogs;
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LogsQuery {
    level: Option<String>,
    query: Option<String>,
    limit: Option<usize>,
}

pub async fn get(query: web::Query<LogsQuery>) -> Result<HttpResponse, ApiError> {
    let limit = query.limit.unwrap_or(200);
    let data = SystemLogs::new(
        "./logs".to_string(),
        query.level.clone(),
        query.query.clone(),
        limit,
    )
    .json()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
