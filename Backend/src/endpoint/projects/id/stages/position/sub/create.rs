use crate::endpoint::api_error::ApiError;
use crate::model::contract::task::Task;
use crate::model::project::stage::appending::StageAppending;
use crate::model::project::stage::id::StageId;
use crate::model::project::stage::invalidating_by_project_id::InvalidatingByProjectId;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Body {
    title: String,
}

pub async fn post(
    state: web::Data<AppState>,
    stage_id: StageId,
    body: Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let project_id = stage_id.project_id();
    let parent_position = stage_id.position();
    InvalidatingByProjectId::new(
        StageAppending::sub(
            state.pool.clone(),
            project_id,
            parent_position,
            body.title.clone(),
        ),
        state.stage_cache.clone(),
        project_id,
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
