use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait ActMedia: Send + Sync + 'static {
    fn add_act(
        &mut self,
        id: Uuid,
        filename: &str,
        mime_type: &str,
        size_bytes: i64,
        created_at: DateTime<Utc>,
        download_url: &str,
    );
}
