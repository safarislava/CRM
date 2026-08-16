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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_added_stages_and_substages() {
        let mut media = CollectingStageMedia::new();
        let project_id = Uuid::new_v4();

        media.add_stage(
            project_id,
            0,
            1,
            "Stage 1",
            None,
            false,
            true,
            Some(100),
            true,
            Some(200),
            false,
            true,
        );

        let items = media.items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].project_id, project_id);
        assert_eq!(items[0].parent_position, 0);
        assert_eq!(items[0].position, 1);
        assert_eq!(items[0].title, "Stage 1");
        assert_eq!(items[0].advance_cost, Some(100));
        assert_eq!(items[0].has_act, true);
    }
}
