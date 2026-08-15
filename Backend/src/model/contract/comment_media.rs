use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait CommentMedia: Send + Sync + 'static {
    fn add_comment(
        &mut self,
        id: Uuid,
        text: &str,
        author: &str,
        is_system: bool,
        created_at: DateTime<Utc>,
        is_pinned: bool,
    );
}
