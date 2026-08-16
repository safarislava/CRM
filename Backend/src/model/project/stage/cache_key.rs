use crate::model::project::id::ProjectId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StageCacheKey {
    ByProjectId(ProjectId),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn supports_equality_and_hashing_for_stage_cache_keys() {
        let p_id = ProjectId::new(Uuid::new_v4());
        let k = StageCacheKey::ByProjectId(p_id);

        let mut map = HashMap::new();
        map.insert(k.clone(), "stages");

        assert_eq!(map.get(&StageCacheKey::ByProjectId(p_id)), Some(&"stages"));
    }
}
