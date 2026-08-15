use crate::model::user::admin::Admin;
use crate::model::user::user::UserId;
use actix_web::{HttpMessage, HttpRequest};

pub trait AuthHeader {
    fn user(&self) -> Option<UserId>;
    fn admin(&self) -> Option<Admin>;
}

impl AuthHeader for HttpRequest {
    fn user(&self) -> Option<UserId> {
        self.extensions().get::<UserId>().cloned()
    }

    fn admin(&self) -> Option<Admin> {
        self.extensions().get::<Admin>().cloned()
    }
}
