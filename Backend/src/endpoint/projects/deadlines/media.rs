use crate::model::contract::deadline_media::DeadlineMedia;
use crate::model::project::stage::id::StageId;
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
    stage_id: StageItemMedia,
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
        stage_id: StageId,
        title: &str,
        deadline: Option<DateTime<Utc>>,
        completed: bool,
        project_title: &str,
    ) {
        self.items.push(JsonDeadlineItem {
            stage_id: StageItemMedia {
                project_id: stage_id.project_id().id(),
                parent_position: stage_id.parent_position(),
                position: stage_id.position(),
                title: title.to_string(),
                deadline,
                completed,
            },
            project_title: project_title.to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::project::id::ProjectId;

    #[test]
    fn serializes_deadline_media_to_json() {
        let mut media = JsonDeadlineMedia::default();
        let p_id = ProjectId::from(Uuid::nil());
        let stage_id = StageId::new_substage(p_id, 0, 1);
        let now = Utc::now();

        media.add_deadline(
            stage_id,
            "Foundation Work",
            Some(now),
            false,
            "Office Building",
        );

        let json = serde_json::to_string(&media).unwrap();
        assert!(json.contains("\"project_title\":\"Office Building\""));
        assert!(json.contains("\"title\":\"Foundation Work\""));
    }
}
