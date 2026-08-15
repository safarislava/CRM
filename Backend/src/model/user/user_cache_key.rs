use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UserCacheKey {
    ByUsername(String),
    ById(Uuid),
}
