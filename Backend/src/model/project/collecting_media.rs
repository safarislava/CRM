use crate::model::contract::project_media::ProjectMedia;
use crate::model::project::cached_summaries::ProjectSummaryItem;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct CollectingProjectMedia {
    items: Vec<ProjectSummaryItem>,
}

impl CollectingProjectMedia {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(self) -> Vec<ProjectSummaryItem> {
        self.items
    }
}

impl ProjectMedia for CollectingProjectMedia {
    fn add_project(&mut self, id: Uuid, title: &str, updated_at: DateTime<Utc>) {
        self.items.push(ProjectSummaryItem {
            id,
            title: title.to_string(),
            updated_at,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_added_projects() {
        let mut media = CollectingProjectMedia::new();
        let uuid = Uuid::new_v4();
        let now = Utc::now();

        media.add_project(uuid, "Project Alpha", now);
        let items = media.items();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, uuid);
        assert_eq!(items[0].title, "Project Alpha");
        assert_eq!(items[0].updated_at, now);
    }
}
