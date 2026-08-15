use crate::model::contract::deadline_media::DeadlineMedia;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct StageItemMedia {
    project_id: Uuid,
    parent_position: i32,
    position: i32,
    title: String,
    deadline: Option<DateTime<Utc>>,
    completed: bool,
}

#[derive(Serialize)]
struct JsonDeadlineItem {
    stage: StageItemMedia,
    project_title: String,
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct JsonDeadlineMedia {
    items: Vec<JsonDeadlineItem>,
}

impl DeadlineMedia for JsonDeadlineMedia {
    fn add_deadline(
        &mut self,
        project_id: Uuid,
        parent_position: i32,
        position: i32,
        title: &str,
        deadline: Option<DateTime<Utc>>,
        completed: bool,
        project_title: &str,
    ) {
        self.items.push(JsonDeadlineItem {
            stage: StageItemMedia {
                project_id,
                parent_position,
                position,
                title: title.to_string(),
                deadline,
                completed,
            },
            project_title: project_title.to_string(),
        });
    }
}
