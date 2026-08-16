use crate::model::contract::project_media::ProjectMedia;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct JsonProjectItem {
    id: Uuid,
    title: String,
    updated_at: DateTime<Utc>,
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct JsonProjectMedia {
    items: Vec<JsonProjectItem>,
}

impl ProjectMedia for JsonProjectMedia {
    fn add_project(&mut self, id: Uuid, title: &str, updated_at: DateTime<Utc>) {
        self.items.push(JsonProjectItem {
            id,
            title: title.to_string(),
            updated_at,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_project_media_to_json() {
        let mut media = JsonProjectMedia::default();
        let uuid = Uuid::nil();
        let now = Utc::now();

        media.add_project(uuid, "Project 1", now);

        let json = serde_json::to_string(&media).unwrap();
        assert!(json.contains("\"title\":\"Project 1\""));
        assert!(json.contains(&uuid.to_string()));
    }
}
