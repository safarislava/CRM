use crate::endpoint::api_error::ApiError;
use crate::model::contract::task::Task;
use crate::model::project::stage::comment::id::CommentId;
use crate::model::project::stage::comment::removal::CommentRemoval;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, i32, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let (_, _, _, comment_id) = path.into_inner();
    CommentRemoval::new(state.pool.clone(), CommentId::new(comment_id))
        .perform()
        .await
        .map_err(|_| ApiError::NotFound("Comment not found".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
