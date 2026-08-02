use crate::middleware::admin_middleware_service::AdminMiddlewareService;
use actix_web::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use std::future::{Ready, ready};
use std::rc::Rc;

pub struct AdminMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AdminMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AdminMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminMiddlewareService {
            service: Rc::new(service),
        }))
    }
}
