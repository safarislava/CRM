use crate::model::project::id::ProjectId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StageCacheKey {
    ByProjectId(ProjectId),
}
