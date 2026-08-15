use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProjectCacheKey {
    AllSummaries,
    ById(Uuid),
}
