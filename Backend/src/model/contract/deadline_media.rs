use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait DeadlineMedia: Send + Sync + 'static {
    fn add_deadline(
        &mut self,
        project_id: Uuid,
        parent_position: i32,
        position: i32,
        title: &str,
        deadline: Option<DateTime<Utc>>,
        completed: bool,
        project_title: &str,
    );
}
