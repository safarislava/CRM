use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
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

impl From<Uuid> for ProjectId {
    fn from(id: Uuid) -> Self {
        ProjectId::new(id)
    }
}

impl From<ProjectId> for Uuid {
    fn from(project_id: ProjectId) -> Self {
        project_id.id
    }
}

impl std::fmt::Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
