use crate::model::project::stage::id::StageId;
use chrono::{DateTime, Utc};

pub trait DeadlineMedia: Send + Sync + 'static {
    fn add_deadline(
        &mut self,
        stage_id: StageId,
        title: &str,
        deadline: Option<DateTime<Utc>>,
        completed: bool,
        project_title: &str,
    );
}
