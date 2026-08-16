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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;
    use uuid::Uuid;

    #[actix_web::test]
    async fn extracts_project_id_from_valid_path() {
        let uuid = Uuid::new_v4();
        let req = TestRequest::default()
            .param("project_id", uuid.to_string())
            .to_http_request();

        let mut payload = Payload::None;
        let res = ProjectId::from_request(&req, &mut payload).await;

        assert_eq!(res.unwrap(), ProjectId::new(uuid));
    }

    #[actix_web::test]
    async fn returns_error_for_missing_or_invalid_path() {
        let req = TestRequest::default()
            .param("project_id", "invalid-uuid")
            .to_http_request();

        let mut payload = Payload::None;
        let res = ProjectId::from_request(&req, &mut payload).await;

        assert!(matches!(res, Err(ApiError::BadRequest(_))));
    }
}
