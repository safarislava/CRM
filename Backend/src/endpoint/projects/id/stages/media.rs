use crate::model::contract::stage_media::StageMedia;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct JsonStageItem {
    project_id: Uuid,
    parent_position: i32,
    position: i32,
    title: String,
    deadline: Option<DateTime<Utc>>,
    completed: bool,
    gip_confirmed: bool,
    advance_cost: Option<i32>,
    advance_confirmed: bool,
    final_cost: Option<i32>,
    final_confirmed: bool,
    has_act: bool,
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct JsonStageMedia {
    items: Vec<JsonStageItem>,
}

impl StageMedia for JsonStageMedia {
    fn add_stage(
        &mut self,
        project_id: Uuid,
        parent_position: i32,
        position: i32,
        title: &str,
        deadline: Option<DateTime<Utc>>,
        completed: bool,
        gip_confirmed: bool,
        advance_cost: Option<i32>,
        advance_confirmed: bool,
        final_cost: Option<i32>,
        final_confirmed: bool,
        has_act: bool,
    ) {
        self.items.push(JsonStageItem {
            project_id,
            parent_position,
            position,
            title: title.to_string(),
            deadline,
            completed,
            gip_confirmed,
            advance_cost,
            advance_confirmed,
            final_cost,
            final_confirmed,
            has_act,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_stage_media_to_json() {
        let mut media = JsonStageMedia::default();
        let project_id = Uuid::nil();

        media.add_stage(
            project_id,
            0,
            1,
            "Concept Design",
            None,
            false,
            true,
            Some(50000),
            true,
            Some(100000),
            false,
            true,
        );

        let json = serde_json::to_string(&media).unwrap();
        assert!(json.contains("\"title\":\"Concept Design\""));
        assert!(json.contains("\"advance_cost\":50000"));
        assert!(json.contains("\"has_act\":true"));
    }
}
