use crate::model::credential::contract::username::{Username, UsernameError};
use async_trait::async_trait;

pub struct ValidUsername(Box<dyn Username>);

impl ValidUsername {
    pub fn new(username: impl Username) -> Self {
        Self(Box::new(username))
    }
}

#[async_trait]
impl Username for ValidUsername {
    async fn value(&self) -> Result<String, UsernameError> {
        let content = self.0.value().await?;
        let len = content.chars().count();
        if len < 3 {
            return Err(UsernameError::TooShort);
        }
        if len > 50 {
            return Err(UsernameError::TooLong);
        }
        if !content.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || c == '_'
                || c == '-'
                || c == ' '
                || ('\u{0400}'..='\u{04FF}').contains(&c)
        }) {
            return Err(UsernameError::InvalidChars);
        }
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::credential::raw_username::RawUsername;

    #[actix_web::test]
    async fn accepts_valid_alphanumeric_username() {
        let res = ValidUsername::new(RawUsername::new("john_doe-12".to_string()))
            .value()
            .await;
        assert_eq!(res.unwrap(), "john_doe-12");
    }

    #[actix_web::test]
    async fn accepts_valid_cyrillic_username() {
        let res = ValidUsername::new(RawUsername::new("Иван_Иванов".to_string()))
            .value()
            .await;
        assert_eq!(res.unwrap(), "Иван_Иванов");
    }

    #[actix_web::test]
    async fn rejects_too_short_username() {
        let res = ValidUsername::new(RawUsername::new("ab".to_string()))
            .value()
            .await;
        assert!(matches!(res, Err(UsernameError::TooShort)));
    }

    #[actix_web::test]
    async fn rejects_too_long_username() {
        let res = ValidUsername::new(RawUsername::new("a".repeat(51)))
            .value()
            .await;
        assert!(matches!(res, Err(UsernameError::TooLong)));
    }

    #[actix_web::test]
    async fn rejects_invalid_characters() {
        let res = ValidUsername::new(RawUsername::new("user@name!".to_string()))
            .value()
            .await;
        assert!(matches!(res, Err(UsernameError::InvalidChars)));
    }
}
