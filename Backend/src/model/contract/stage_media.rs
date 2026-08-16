use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait StageMedia: Send + Sync + 'static {
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
    );
}
