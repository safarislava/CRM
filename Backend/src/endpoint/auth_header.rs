use crate::model::user::admin::Admin;
use crate::model::user::user::User;
use actix_web::{HttpMessage, HttpRequest};

pub trait AuthHeader {
    fn user(&self) -> Option<User>;
    fn admin(&self) -> Option<Admin>;
}

impl AuthHeader for HttpRequest {
    fn user(&self) -> Option<User> {
        self.extensions().get::<User>().cloned()
    }

    fn admin(&self) -> Option<Admin> {
        self.extensions().get::<Admin>().cloned()
    }
}
