use crate::model::contract::box_error::BoxError;
use crate::model::project::contract::file::File;
use crate::storage::Storage;
use aws_sdk_s3::primitives::ByteStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use tokio::sync::Mutex;

pub struct StreamedFile {
    filename: String,
    mime_type: String,
    stream: Mutex<Option<ByteStream>>,
    size_bytes: Arc<AtomicI64>,
}

impl StreamedFile {
    pub fn new(
        filename: String,
        mime_type: String,
        stream: ByteStream,
        size_bytes: Arc<AtomicI64>,
    ) -> Self {
        Self {
            filename,
            mime_type,
            stream: Mutex::new(Some(stream)),
            size_bytes,
        }
    }
}

#[async_trait::async_trait]
impl File for StreamedFile {
    fn name(&self) -> &str {
        &self.filename
    }

    fn media_type(&self) -> &str {
        &self.mime_type
    }

    fn size_bytes(&self) -> i64 {
        self.size_bytes.load(Ordering::SeqCst)
    }

    async fn upload_to(&self, storage: &Storage, key: &str) -> Result<(), BoxError> {
        let stream = self
            .stream
            .lock()
            .await
            .take()
            .ok_or_else(|| "ByteStream already consumed".to_string())?;
        storage
            .upload_stream(key, stream, &self.mime_type, &self.filename, None)
            .await
    }
}
