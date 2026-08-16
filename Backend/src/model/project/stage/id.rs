use crate::model::project::id::ProjectId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StageId {
    project_id: ProjectId,
    parent_position: i32,
    position: i32,
}

impl StageId {
    pub fn new(project_id: ProjectId, position: i32) -> Self {
        StageId {
            project_id,
            parent_position: 0,
            position,
        }
    }

    pub fn new_substage(project_id: ProjectId, parent_position: i32, position: i32) -> Self {
        StageId {
            project_id,
            parent_position,
            position,
        }
    }

    pub fn project_id(&self) -> ProjectId {
        self.project_id
    }

    pub fn parent_position(&self) -> i32 {
        self.parent_position
    }

    pub fn position(&self) -> i32 {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn creates_top_level_stage_id() {
        let project_id = ProjectId::new(Uuid::new_v4());
        let stage_id = StageId::new(project_id, 3);

        assert_eq!(stage_id.project_id(), project_id);
        assert_eq!(stage_id.parent_position(), 0);
        assert_eq!(stage_id.position(), 3);
    }

    #[test]
    fn creates_substage_id() {
        let project_id = ProjectId::new(Uuid::new_v4());
        let stage_id = StageId::new_substage(project_id, 2, 5);

        assert_eq!(stage_id.project_id(), project_id);
        assert_eq!(stage_id.parent_position(), 2);
        assert_eq!(stage_id.position(), 5);
    }

    #[test]
    fn supports_equality_and_hashing() {
        let project_id = ProjectId::new(Uuid::new_v4());
        let s1 = StageId::new_substage(project_id, 1, 2);
        let s2 = StageId::new_substage(project_id, 1, 2);

        let mut map = HashMap::new();
        map.insert(s1, "substage");

        assert_eq!(s1, s2);
        assert_eq!(map.get(&s2), Some(&"substage"));
    }
}
