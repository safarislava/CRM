use crate::model::session::contract::user_id_source::UserIdSource;
use crate::model::session::signed_access_token::SignedAccessToken;
use crate::model::user::id::UserId;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, forward_ready};
use actix_web::{Error, HttpMessage};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

pub struct JwtMiddlewareService<S> {
    pub service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        let user_id = request
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .and_then(|token| SignedAccessToken::new(token.to_string()).user_id());

        match user_id {
            Some(id) => {
                request.extensions_mut().insert(UserId::new(id));
                Box::pin(svc.call(request))
            }
            None => {
                let path = request.path().to_string();
                tracing::warn!(path = %path, "Unauthorized access attempt: missing or invalid JWT token");
                Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jwt::jwt_secret;
    use crate::model::session::claims::Claims;
    use actix_web::http::StatusCode;
    use actix_web::http::header;
    use actix_web::test::TestRequest;
    use jsonwebtoken::{EncodingKey, Header, encode};
    use uuid::Uuid;

    #[actix_web::test]
    async fn rejects_request_without_authorization_header() {
        let req = TestRequest::default().to_srv_request();
        let svc = Rc::new(actix_web::test::ok_service());
        let middleware = JwtMiddlewareService { service: svc };

        let res = middleware.call(req).await;
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(err.error_response().status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn allows_request_with_valid_jwt_header() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(user_id, Uuid::new_v4(), "access".to_string(), 9999999999);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret().as_bytes()),
        )
        .unwrap();

        let req = TestRequest::default()
            .insert_header((header::AUTHORIZATION, format!("Bearer {token}")))
            .to_srv_request();

        let svc = Rc::new(actix_web::test::ok_service());
        let middleware = JwtMiddlewareService { service: svc };

        let res = middleware.call(req).await;
        assert!(res.is_ok());
    }
}
