use crate::endpoint::api_error::ApiError;
use crate::model::admin::user_role_update::UserRoleUpdate;
use crate::model::task::contract::task::Task;
use crate::model::user::role::Role;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RolesUpdateBody {
    roles: Vec<Role>,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<RolesUpdateBody>,
) -> Result<HttpResponse, ApiError> {
    let target_user_id = path.into_inner();
    UserRoleUpdate::new(state.pool.clone(), target_user_id, body.into_inner().roles)
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().finish())
}
