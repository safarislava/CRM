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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn creates_and_converts_project_id() {
        let uuid = Uuid::new_v4();
        let project_id = ProjectId::new(uuid);
        let converted_uuid: Uuid = project_id.into();

        assert_eq!(project_id.id(), uuid);
        assert_eq!(converted_uuid, uuid);
        assert_eq!(ProjectId::from(uuid), project_id);
        assert_eq!(format!("{project_id}"), uuid.to_string());
    }

    #[test]
    fn supports_equality_and_hashing() {
        let uuid = Uuid::new_v4();
        let id1 = ProjectId::new(uuid);
        let id2 = ProjectId::new(uuid);

        let mut map = HashMap::new();
        map.insert(id1, "test_project");

        assert_eq!(id1, id2);
        assert_eq!(map.get(&id2), Some(&"test_project"));
    }
}
