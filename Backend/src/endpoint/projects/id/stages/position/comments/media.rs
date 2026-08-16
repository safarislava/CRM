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
