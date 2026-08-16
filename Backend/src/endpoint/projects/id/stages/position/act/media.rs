use crate::model::contract::act_media::ActMedia;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct JsonActItem {
    id: Uuid,
    filename: String,
    mime_type: String,
    size_bytes: i64,
    created_at: DateTime<Utc>,
    download_url: String,
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct JsonActMedia {
    items: Vec<JsonActItem>,
}

impl ActMedia for JsonActMedia {
    fn add_act(
        &mut self,
        id: Uuid,
        filename: &str,
        mime_type: &str,
        size_bytes: i64,
        created_at: DateTime<Utc>,
        download_url: &str,
    ) {
        self.items.push(JsonActItem {
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
    fn serializes_act_media_to_json() {
        let mut media = JsonActMedia::default();
        let act_id = Uuid::nil();
        let now = Utc::now();

        media.add_act(
            act_id,
            "signed_act.pdf",
            "application/pdf",
            1024,
            now,
            "/api/download/act/123",
        );

        let json = serde_json::to_string(&media).unwrap();
        assert!(json.contains("\"filename\":\"signed_act.pdf\""));
        assert!(json.contains("\"download_url\":\"/api/download/act/123\""));
    }
}
