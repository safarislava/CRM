use crate::model::project::project::ProjectId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StageCacheKey {
    ByProjectId(ProjectId),
    ByPosition {
        project_id: ProjectId,
        parent_position: i32,
        position: i32,
    },
}
