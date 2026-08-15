#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UserCacheKey {
    ByUsername(String),
}
