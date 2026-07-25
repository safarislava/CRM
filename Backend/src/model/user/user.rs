use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    id: Uuid,
}

impl User {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
