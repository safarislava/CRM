use crate::endpoint::api_error::ApiError;
use crate::endpoint::json_project_media::JsonProjectMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::project_summaries::ProjectSummaries;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn get(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let mut media = JsonProjectMedia::default();
    ProjectSummaries::new(state.pool.clone())
        .print(&mut media)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}