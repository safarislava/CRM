use crate::model::contract::box_error::BoxError;
use crate::model::project::contract::file::File;
use crate::storage::Storage;

#[derive(Clone)]
pub struct FileContent {
    filename: String,
    mime_type: String,
    data: Vec<u8>,
}

impl FileContent {
    pub fn new(filename: String, mime_type: String, data: Vec<u8>) -> Self {
        Self {
            filename,
            mime_type,
            data,
        }
    }
}

#[async_trait::async_trait]
impl File for FileContent {
    fn name(&self) -> &str {
        &self.filename
    }

    fn media_type(&self) -> &str {
        &self.mime_type
    }

    fn size_bytes(&self) -> i64 {
        self.data.len() as i64
    }

    async fn upload_to(&self, storage: &Storage, key: &str) -> Result<(), BoxError> {
        storage
            .upload(key, self.data.clone(), &self.mime_type, &self.filename)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_file_attributes() {
        let payload = vec![1, 2, 3, 4, 5];
        let file = FileContent::new(
            "act.pdf".to_string(),
            "application/pdf".to_string(),
            payload,
        );

        assert_eq!(file.name(), "act.pdf");
        assert_eq!(file.media_type(), "application/pdf");
        assert_eq!(file.size_bytes(), 5);
    }
}
