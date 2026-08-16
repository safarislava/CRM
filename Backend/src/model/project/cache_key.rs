use crate::model::project::id::ProjectId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProjectCacheKey {
    AllSummaries,
    ByProjectId(ProjectId),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn supports_equality_and_hashing_for_project_cache_keys() {
        let p_id = ProjectId::new(Uuid::new_v4());
        let k1 = ProjectCacheKey::AllSummaries;
        let k2 = ProjectCacheKey::ByProjectId(p_id);

        let mut map = HashMap::new();
        map.insert(k1.clone(), "all");
        map.insert(k2.clone(), "by_id");

        assert_eq!(map.get(&ProjectCacheKey::AllSummaries), Some(&"all"));
        assert_eq!(map.get(&ProjectCacheKey::ByProjectId(p_id)), Some(&"by_id"));
    }
}
