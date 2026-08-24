use crate::model::user::admin::authority::AdminAuthority;
use crate::model::user::id::UserId;
use actix_web::{HttpMessage, HttpRequest};

pub trait AuthHeader {
    fn user(&self) -> Option<UserId>;
    fn admin(&self) -> Option<AdminAuthority>;
}

impl AuthHeader for HttpRequest {
    fn user(&self) -> Option<UserId> {
        self.extensions().get::<UserId>().cloned()
    }

    fn admin(&self) -> Option<AdminAuthority> {
        self.extensions().get::<AdminAuthority>().cloned()
    }
}
