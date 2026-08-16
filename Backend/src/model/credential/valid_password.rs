use crate::model::credential::contract::password::{Password, PasswordError};

pub struct ValidPassword(Box<dyn Password>);

impl ValidPassword {
    pub fn new(password: impl Password) -> Self {
        Self(Box::new(password))
    }
}

impl Password for ValidPassword {
    fn value(&self) -> Result<String, PasswordError> {
        let content = self.0.value()?;
        let len = content.len();
        if len < 6 {
            return Err(PasswordError::TooShort);
        }
        if len > 72 {
            return Err(PasswordError::TooLong);
        }
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::credential::raw_password::RawPassword;

    #[test]
    fn accepts_valid_password() {
        assert_eq!(
            ValidPassword::new(RawPassword::new("secret123".to_string())).value(),
            Ok("secret123".to_string())
        );
    }

    #[test]
    fn rejects_too_short_password() {
        assert_eq!(
            ValidPassword::new(RawPassword::new("12345".to_string())).value(),
            Err(PasswordError::TooShort)
        );
    }

    #[test]
    fn rejects_too_long_password() {
        assert_eq!(
            ValidPassword::new(RawPassword::new("a".repeat(73))).value(),
            Err(PasswordError::TooLong)
        );
    }
}
