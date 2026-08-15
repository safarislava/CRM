use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StageCacheKey {
    ByProjectId(Uuid),
    ByPosition {
        project_id: Uuid,
        parent_position: i32,
        position: i32,
    },
}
