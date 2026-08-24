use crate::model::user::admin::verification::VerificationAdmin;
use crate::model::user::contract::admin_access::AdminAccess;
use crate::model::user::id::UserId;
use crate::state::AppState;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, forward_ready};
use actix_web::{Error, HttpMessage, web};
use std::pin::Pin;
use std::rc::Rc;

pub struct AdminMiddlewareService<S> {
    pub service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AdminMiddlewareService<S>
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
        let user = request.extensions().get::<UserId>().cloned();
        let state = request.app_data::<web::Data<AppState>>().cloned();

        Box::pin(async move {
            let (Some(user), Some(state)) = (user, state) else {
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
            };

            match VerificationAdmin::new(user, state.pool.clone())
                .admin()
                .await
            {
                Ok(admin) => {
                    request.extensions_mut().insert(admin);
                    svc.call(request).await
                }
                Err(_) => {
                    let path = request.path().to_string();
                    tracing::warn!(path = %path, "Forbidden access attempt: user is not an administrator");
                    Err(actix_web::error::ErrorForbidden(
                        "Access denied: admin role required",
                    ))
                }
            }
        })
    }
}
