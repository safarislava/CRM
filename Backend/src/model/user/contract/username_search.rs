use crate::model::contract::box_error::BoxError;
use crate::model::credential::contract::username::Username;
use crate::model::user::user::UserId;

#[async_trait::async_trait]
pub trait UsernameSearch {
    async fn found(&self, username: impl Username) -> Result<Option<UserId>, BoxError>;
}
