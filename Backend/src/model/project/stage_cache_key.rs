use uuid::Uuid;
use crate::model::project::project::Project;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StageCacheKey {
    ByProject(Project),
    ByPosition {
        project: Project,
        parent_position: i32,
        position: i32,
    },
}
