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
