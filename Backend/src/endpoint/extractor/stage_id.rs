use crate::endpoint::api_error::ApiError;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::id::StageId;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, web};
use futures_util::future::{Ready, ready};

impl FromRequest for StageId {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Ok(path) = web::Path::<(ProjectId, i32, i32)>::extract(req).into_inner() {
            let (project_id, parent_position, position) = path.into_inner();
            return ready(Ok(StageId::new_substage(
                project_id,
                parent_position,
                position,
            )));
        }
        if let Ok(path) = web::Path::<(ProjectId, i32)>::extract(req).into_inner() {
            let (project_id, position) = path.into_inner();
            return ready(Ok(StageId::new(project_id, position)));
        }
        ready(Err(ApiError::BadRequest(
            "Invalid stage id path".to_string(),
        )))
    }
}
