use crate::model::contract::attachment_media::AttachmentMedia;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct JsonAttachmentItem {
    id: Uuid,
    filename: String,
    mime_type: String,
    size_bytes: i64,
    created_at: DateTime<Utc>,
    download_url: String,
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct JsonAttachmentMedia {
    items: Vec<JsonAttachmentItem>,
}

impl AttachmentMedia for JsonAttachmentMedia {
    fn add_attachment(
        &mut self,
        id: Uuid,
        filename: &str,
        mime_type: &str,
        size_bytes: i64,
        created_at: DateTime<Utc>,
        download_url: &str,
    ) {
        self.items.push(JsonAttachmentItem {
            id,
            filename: filename.to_string(),
            mime_type: mime_type.to_string(),
            size_bytes,
            created_at,
            download_url: download_url.to_string(),
        });
    }
}
