use crate::model::contract::comment_media::CommentMedia;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct JsonCommentItem {
    id: Uuid,
    text: String,
    author: String,
    is_system: bool,
    created_at: DateTime<Utc>,
    is_pinned: bool,
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct JsonCommentMedia {
    items: Vec<JsonCommentItem>,
}

impl CommentMedia for JsonCommentMedia {
    fn add_comment(
        &mut self,
        id: Uuid,
        text: &str,
        author: &str,
        is_system: bool,
        created_at: DateTime<Utc>,
        is_pinned: bool,
    ) {
        self.items.push(JsonCommentItem {
            id,
            text: text.to_string(),
            author: author.to_string(),
            is_system,
            created_at,
            is_pinned,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_comment_media_to_json() {
        let mut media = JsonCommentMedia::default();
        let c_id = Uuid::nil();
        let now = Utc::now();

        media.add_comment(c_id, "Note added", "Alice", false, now, true);

        let json = serde_json::to_string(&media).unwrap();
        assert!(json.contains("\"text\":\"Note added\""));
        assert!(json.contains("\"author\":\"Alice\""));
        assert!(json.contains("\"is_pinned\":true"));
    }
}
