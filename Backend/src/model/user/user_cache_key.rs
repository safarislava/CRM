use crate::model::user::user::UserId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UserCacheKey {
    ByUsername(String),
    ById(UserId),
}
