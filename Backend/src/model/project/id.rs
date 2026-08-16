use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProjectId {
    id: Uuid,
}

impl ProjectId {
    pub fn new(id: Uuid) -> Self {
        ProjectId { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
