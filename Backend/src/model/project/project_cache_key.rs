use crate::model::project::project::ProjectId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProjectCacheKey {
    AllSummaries,
    ByProjectId(ProjectId),
}
