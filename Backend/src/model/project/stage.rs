use crate::model::project::project::ProjectId;

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
