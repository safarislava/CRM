use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::AuthHeader;
use crate::model::credential::raw_username::RawUsername;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::contract::task::Task;
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
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let username = ValidUsername::new(RawUsername::new(body.username.clone()));
    UsernameUpdate::new(state.pool.clone(), user, username)
        .perform()
        .await?;
    Ok(HttpResponse::Ok().finish())
}
