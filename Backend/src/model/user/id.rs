use std::fmt;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId {
    id: Uuid,
}

impl UserId {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn creates_and_formats_user_id() {
        let uuid = Uuid::new_v4();
        let user_id = UserId::new(uuid);

        assert_eq!(user_id.id(), uuid);
        assert_eq!(format!("{user_id}"), uuid.to_string());
    }

    #[test]
    fn supports_equality_and_hashing() {
        let uuid = Uuid::new_v4();
        let u1 = UserId::new(uuid);
        let u2 = UserId::new(uuid);

        let mut map = HashMap::new();
        map.insert(u1, "admin_user");

        assert_eq!(u1, u2);
        assert_eq!(map.get(&u2), Some(&"admin_user"));
    }
}
