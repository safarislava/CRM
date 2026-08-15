use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait ProjectMedia: Send + Sync + 'static {
    fn add_project(&mut self, id: Uuid, title: &str, updated_at: DateTime<Utc>);
}
