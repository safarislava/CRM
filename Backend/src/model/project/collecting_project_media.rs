use crate::model::contract::project_media::ProjectMedia;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::model::project::cached_project_summaries::ProjectSummaryItem;

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
