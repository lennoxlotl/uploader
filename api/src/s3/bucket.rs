use async_trait::async_trait;

use aws_sdk_s3::{
    error::SdkError,
    operation::{
        get_object::{GetObjectError, GetObjectOutput},
        put_object::{PutObjectError, PutObjectOutput},
    },
    primitives::ByteStream,
    Client, Config,
};

use super::credentials::BucketCredentials;

/// Provides access to object-based storage buckets that are accessible using amazons s3 api spec
#[derive(Debug, Clone)]
pub struct Bucket {
    client: Client,
    name: String,
}

#[async_trait]
pub trait BucketOperations {
    /// Gets an item from the bucket
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the object
    ///
    /// # Returns
    /// The object
    async fn get(&self, key: &str) -> Result<GetObjectOutput, SdkError<GetObjectError>>;

    /// Puts an item into the bucket
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the object
    /// * `bytes` - The bytes of the object to be created
    ///
    /// # Returns
    /// Result of the insert
    async fn put<'a>(
        &self,
        key: &str,
        bytes: ByteStream,
        content_type: Option<&'a str>,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError>>;
}

impl Bucket {
    /// Creates a new bucket access
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the bucket
    /// * `endpoint` - The endpoint of the bucket (url to access)
    /// * `credentials` - The credentials required to access the bucket
    ///
    /// # Returns
    /// The configured bucket
    pub fn new(name: String, endpoint: String, credentials: BucketCredentials) -> Self {
        Self {
            client: Client::from_conf(
                Config::builder()
                    .endpoint_url(endpoint)
                    .region(credentials.region)
                    .credentials_provider(credentials.credentials)
                    .force_path_style(true)
                    .behavior_version_latest()
                    .build(),
            ),
            name,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[async_trait]
impl BucketOperations for Bucket {
    async fn get(&self, key: &str) -> Result<GetObjectOutput, SdkError<GetObjectError>> {
        self.client
            .get_object()
            .bucket(self.name())
            .key(key)
            .send()
            .await
    }

    async fn put<'a>(
        &self,
        key: &str,
        bytes: ByteStream,
        content_type: Option<&'a str>,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        self.client
            .put_object()
            .bucket(self.name())
            .key(key)
            .body(bytes)
            .content_type(content_type.unwrap_or("application/octet-stream"))
            .send()
            .await
    }
}
