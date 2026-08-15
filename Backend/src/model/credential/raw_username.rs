use crate::model::credential::contract::username::{Username, UsernameError};
use async_trait::async_trait;

pub struct RawUsername(String);

impl RawUsername {
    pub fn new(username: String) -> Self {
        Self(username)
    }
}

#[async_trait]
impl Username for RawUsername {
    async fn value(&self) -> Result<String, UsernameError> {
        Ok(self.0.clone())
    }
}
