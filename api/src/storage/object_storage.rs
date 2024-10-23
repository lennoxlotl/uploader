use crate::s3::bucket::{Bucket, BucketOperations};

use super::driver::{StorageError, StorageResult};
use aws_sdk_s3::primitives::ByteStream;

/// Implements save_file for the ObjectStorage type
pub(crate) async fn save_file(
    bucket: &Bucket,
    id: &str,
    content_type: &str,
    bytes: Vec<u8>,
) -> StorageResult<()> {
    bucket
        .put(&id, ByteStream::from(bytes), Some(&content_type))
        .await
        .map_err(|_| StorageError::BucketSaveError)
        .map(|_| ())
}

/// Implements get_file for the ObjectStorage type
pub(crate) async fn get_file(bucket: &Bucket, id: &str) -> StorageResult<(Vec<u8>, String)> {
    let data = bucket.get(&id).await.unwrap();
    let file_type = &data.content_type.ok_or(StorageError::BucketSaveError)?;
    let file_bytes = data
        .body
        .collect()
        .await
        .map_err(|_| StorageError::BucketLoadError)?
        .to_vec();
    Ok((file_bytes, file_type.to_string()))
}

/// Implements delete_file for the ObjectStorage type
pub(crate) async fn delete_file(bucket: &Bucket, id: &str) -> StorageResult<()> {
    bucket
        .delete(&id)
        .await
        .map_err(|_| StorageError::BucketDeleteError)
        .map(|_| ())
}
