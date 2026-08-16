use crate::model::credential::contract::hash::{Hash, HashError};
use crate::model::credential::contract::password::Password;

pub struct HashedPassword(Box<dyn Password>);

impl HashedPassword {
    pub fn new(password: impl Password) -> Self {
        Self(Box::new(password))
    }
}

#[async_trait::async_trait]
impl Hash for HashedPassword {
    async fn value(&self) -> Result<String, HashError> {
        let raw = self
            .0
            .value()
            .map_err(|e| HashError::Internal(Box::new(e)))?;
        actix_web::rt::task::spawn_blocking(move || {
            bcrypt::hash(&raw, bcrypt::DEFAULT_COST).map_err(|_| HashError::Bcrypt)
        })
        .await
        .map_err(|_| HashError::Task)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::credential::raw_password::RawPassword;

    #[actix_web::test]
    async fn produces_valid_bcrypt_hash() {
        let raw = "my_secure_pass";
        let hash = HashedPassword::new(RawPassword::new(raw.to_string()))
            .value()
            .await
            .unwrap();
        assert!(bcrypt::verify(raw, &hash).unwrap());
    }
}
