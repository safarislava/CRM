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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_attachment_media_to_json() {
        let mut media = JsonAttachmentMedia::default();
        let att_id = Uuid::nil();
        let now = Utc::now();

        media.add_attachment(
            att_id,
            "blueprint.dwg",
            "application/acad",
            204800,
            now,
            "/api/download/123",
        );

        let json = serde_json::to_string(&media).unwrap();
        assert!(json.contains("\"filename\":\"blueprint.dwg\""));
        assert!(json.contains("\"size_bytes\":204800"));
        assert!(json.contains("\"download_url\":\"/api/download/123\""));
    }
}
