use crate::model::contract::box_error::BoxError;
use crate::model::session::contract::cookie::Cookie;
use crate::model::session::contract::token::Token;
use actix_web::cookie::time::Duration;

pub struct CookieToken {
    name: String,
    token: Box<dyn Token>,
    max_age: Duration,
}

impl CookieToken {
    pub fn new(name: String, token: Box<dyn Token>, max_age: Duration) -> Self {
        Self {
            name,
            token,
            max_age,
        }
    }
}

#[async_trait::async_trait]
impl Cookie for CookieToken {
    async fn value(&self) -> Result<actix_web::cookie::Cookie<'static>, BoxError> {
        Ok(
            actix_web::cookie::Cookie::build(self.name.clone(), self.token.value().await?)
                .http_only(true)
                .secure(true)
                .same_site(actix_web::cookie::SameSite::Strict)
                .path("/api/auth")
                .max_age(self.max_age)
                .finish(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyToken(&'static str);

    #[async_trait::async_trait]
    impl Token for DummyToken {
        async fn value(&self) -> Result<String, BoxError> {
            Ok(self.0.to_string())
        }
    }

    #[actix_web::test]
    async fn creates_secure_cookie() {
        let cookie = CookieToken::new(
            "refresh_token".to_string(),
            Box::new(DummyToken("dummy_val")),
            Duration::days(7),
        )
        .value()
        .await
        .unwrap();

        assert_eq!(cookie.name(), "refresh_token");
        assert_eq!(cookie.value(), "dummy_val");
        assert_eq!(cookie.path(), Some("/api/auth"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.secure(), Some(true));
        assert_eq!(
            cookie.same_site(),
            Some(actix_web::cookie::SameSite::Strict)
        );
    }
}
