use crate::model::user::admin::Admin;
use crate::model::user::user::User;
use actix_web::{HttpMessage, HttpRequest};

pub trait UserHeader {
    fn user(&self) -> Option<User>;
    fn admin(&self) -> Option<Admin>;
}

impl UserHeader for HttpRequest {
    fn user(&self) -> Option<User> {
        self.extensions().get::<User>().cloned()
    }

    fn admin(&self) -> Option<Admin> {
        self.extensions().get::<Admin>().cloned()
    }
}
