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
