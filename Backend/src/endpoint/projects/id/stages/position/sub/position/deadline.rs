use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::contract::task::Task;
use crate::model::project::logged_deadline_update::LoggedDeadlineUpdate;
use crate::model::project::project::ProjectId;
use crate::model::project::stage::StageId;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, web};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    deadline: Option<DateTime<Utc>>,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32, i32)>,
    body: Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, parent_position, position) = path.into_inner();
    LoggedDeadlineUpdate::new(
        state.pool.clone(),
        StageId::new_substage(ProjectId::new(project_id), parent_position, position),
        user,
        body.deadline,
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
