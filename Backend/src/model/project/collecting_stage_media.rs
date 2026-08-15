use crate::model::contract::stage_media::StageMedia;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct StageSummaryItem {
    pub project_id: Uuid,
    pub parent_position: i32,
    pub position: i32,
    pub title: String,
    pub deadline: Option<DateTime<Utc>>,
    pub completed: bool,
    pub gip_confirmed: bool,
    pub advance_cost: Option<i32>,
    pub advance_confirmed: bool,
    pub final_cost: Option<i32>,
    pub final_confirmed: bool,
    pub has_act: bool,
}

#[derive(Clone, Debug, Default)]
pub struct CollectingStageMedia {
    items: Vec<StageSummaryItem>,
}

impl CollectingStageMedia {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(self) -> Vec<StageSummaryItem> {
        self.items
    }
}

impl StageMedia for CollectingStageMedia {
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
        self.items.push(StageSummaryItem {
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
