use crate::model::user::id::UserId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UserCacheKey {
    ByUsername(String),
    ById(UserId),
}
