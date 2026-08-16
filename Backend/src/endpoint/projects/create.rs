use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::audit::AuditAction;
use crate::model::audit::AuditedTask;
use crate::model::contract::task::Task;
use crate::model::project::invalidating_all_summaries::InvalidatingAllSummaries;
use crate::model::project::registration::ProjectRegistration;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct CreateProjectDto {
    title: String,
}

pub async fn create(
    state: web::Data<AppState>,
    request: HttpRequest,
    body: web::Json<CreateProjectDto>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let title = body.title.clone();
    AuditedTask::new(
        user,
        AuditAction::ProjectCreate,
        title.clone(),
        InvalidatingAllSummaries::new(
            ProjectRegistration::new(state.pool.clone(), title),
            state.project_cache.clone(),
        ),
    )
    .perform()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Created().finish())
}
