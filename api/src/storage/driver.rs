use thiserror::Error;

use crate::s3::bucket::Bucket;

use super::object_storage;

pub type StorageResult<T> = std::result::Result<T, StorageError>;

#[derive(Debug, Clone)]
pub enum StorageDriver {
    ObjectStorage { bucket: Bucket },
}

#[derive(Debug, Clone, Error)]
pub enum StorageError {
    #[error("Failed to save file in object storage bucket")]
    BucketSaveError,
    #[error("Failed to load file from object storage bucket")]
    BucketLoadError,
    #[error("Failed to delete file from object storage bucket")]
    BucketDeleteError,
}

impl StorageDriver {
    pub fn object(bucket: Bucket) -> Self {
        Self::ObjectStorage { bucket }
    }

    /// Saves a file in the storage driver
    ///
    /// # Arguments
    ///
    /// * `id` - The file id
    /// * `content_type` - The file type
    /// * `bytes` - The file bytes
    pub async fn save_file(
        &self,
        id: &str,
        content_type: &str,
        bytes: Vec<u8>,
    ) -> StorageResult<()> {
        match self {
            Self::ObjectStorage { bucket } => {
                object_storage::save_file(bucket, id, content_type, bytes).await
            }
        }
    }

    /// Gets a file from the storage driver
    ///
    /// # Arguments
    ///
    /// * `id` - The file id
    ///
    /// # Returns
    ///
    /// The file bytes and content type
    pub async fn get_file(&self, id: &str) -> StorageResult<(Vec<u8>, String)> {
        match self {
            Self::ObjectStorage { bucket } => object_storage::get_file(bucket, id).await,
        }
    }

    /// Deletes a file from the storage driver
    ///
    /// # Arguments
    ///
    /// + `id` - The file id
    pub async fn delete_file(&self, id: &str) -> StorageResult<()> {
        match self {
            Self::ObjectStorage { bucket } => object_storage::delete_file(bucket, id).await,
        }
    }
}
