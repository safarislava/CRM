use crate::model::user::id::UserId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UserCacheKey {
    ByUsername(String),
    ById(UserId),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn supports_equality_and_hashing_for_user_cache_keys() {
        let u_id = UserId::new(Uuid::new_v4());
        let k1 = UserCacheKey::ByUsername("john".to_string());
        let k2 = UserCacheKey::ById(u_id);

        let mut map = HashMap::new();
        map.insert(k1.clone(), "val1");
        map.insert(k2.clone(), "val2");

        assert_eq!(
            map.get(&UserCacheKey::ByUsername("john".to_string())),
            Some(&"val1")
        );
        assert_eq!(map.get(&UserCacheKey::ById(u_id)), Some(&"val2"));
    }
}
