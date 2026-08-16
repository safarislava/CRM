use crate::endpoint::api_error::ApiError;
use crate::model::contract::task::Task;
use crate::model::project::stage::comment::id::CommentId;
use crate::model::project::stage::comment::pinning::CommentPinning;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PinCommentDto {
    pinned: bool,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, Uuid)>,
    body: Json<PinCommentDto>,
) -> Result<HttpResponse, ApiError> {
    let (_, _, comment_id) = path.into_inner();
    CommentPinning::new(state.pool.clone(), CommentId::new(comment_id), body.pinned)
        .perform()
        .await
        .map_err(|_| ApiError::NotFound("Comment not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
