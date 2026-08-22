use crate::endpoint::api_error::ApiError;
use crate::model::project::streamed_file::StreamedFile;
use actix_multipart::Field;
use aws_sdk_s3::primitives::ByteStream;
use futures_util::StreamExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

pub async fn create_streamed_file(
    mut field: Field,
    fallback_filename: &str,
    limit: usize,
) -> Result<Arc<StreamedFile>, ApiError> {
    let filename = field
        .content_disposition()
        .and_then(|cd| cd.get_filename())
        .map(|s| s.to_string())
        .unwrap_or_else(|| fallback_filename.to_string());
    let first_chunk = match field.next().await {
        Some(Ok(bytes)) if !bytes.is_empty() => bytes,
        Some(Ok(_)) => return Err(ApiError::BadRequest("File is empty".to_string())),
        Some(Err(_)) => return Err(ApiError::BadRequest("Invalid multipart data".to_string())),
        None => return Err(ApiError::BadRequest("No file provided".to_string())),
    };
    let mime_type = infer::get(&first_chunk)
        .map(|kind| kind.mime_type().to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());
    let size_counter = Arc::new(AtomicI64::new(0));
    let size_counter_clone = size_counter.clone();
    let (tx, rx) = tokio::sync::mpsc::channel(16);
    let first_len = first_chunk.len() as i64;
    size_counter_clone.store(first_len, Ordering::SeqCst);
    if tx.send(Ok(first_chunk)).await.is_err() {
        return Err(ApiError::Internal("Upload stream closed".to_string()));
    }
    actix_web::rt::spawn(async move {
        while let Some(chunk_res) = field.next().await {
            match chunk_res {
                Ok(chunk) => {
                    let new_total = size_counter_clone
                        .fetch_add(chunk.len() as i64, Ordering::SeqCst)
                        + chunk.len() as i64;
                    if new_total > limit as i64 {
                        let _ = tx
                            .send(Err(std::io::Error::new(
                                std::io::ErrorKind::PermissionDenied,
                                "Payload too large",
                            )))
                            .await;
                        break;
                    }
                    if tx.send(Ok(chunk)).await.is_err() {
                        break;
                    }
                }
                Err(err) => {
                    let _ = tx
                        .send(Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            err.to_string(),
                        )))
                        .await;
                    break;
                }
            }
        }
    });
    let rx_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let frame_stream = rx_stream.map(|res| res.map(http_body::Frame::data));
    let byte_stream = ByteStream::from_body_1_x(http_body_util::StreamBody::new(frame_stream));
    Ok(Arc::new(StreamedFile::new(
        filename,
        mime_type,
        byte_stream,
        size_counter,
    )))
}
