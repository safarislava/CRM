use crate::endpoint::api_error::ApiError;
use crate::model::project::id::ProjectId;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, web};
use futures_util::future::{Ready, ready};

impl FromRequest for ProjectId {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match web::Path::<ProjectId>::extract(req).into_inner() {
            Ok(path) => ready(Ok(path.into_inner())),
            Err(_) => ready(Err(ApiError::BadRequest("Invalid project id".to_string()))),
        }
    }
}
