use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::contract::task::Task;
use crate::model::credential::raw_username::RawUsername;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::user::invalidating_username_update::InvalidatingUsernameUpdate;
use crate::model::user::username_update::UsernameUpdate;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUsernameDto {
    username: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    body: web::Json<UpdateUsernameDto>,
) -> Result<HttpResponse, ApiError> {
    let user_id = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let new_username_str = body.username.clone();
    let username = ValidUsername::new(RawUsername::new(new_username_str.clone()));
    InvalidatingUsernameUpdate::new(
        UsernameUpdate::new(state.pool.clone(), user_id, username),
        state.user_cache.clone(),
        "",
        new_username_str,
    )
    .perform()
    .await?;
    Ok(HttpResponse::Ok().finish())
}
