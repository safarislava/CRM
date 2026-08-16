use crate::model::project::id::ProjectId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProjectCacheKey {
    AllSummaries,
    ByProjectId(ProjectId),
}
