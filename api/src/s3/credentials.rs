use aws_config::Region;
use aws_credential_types::Credentials;

const DEFAULT_REGION: &str = "weur"; // Default Cloudflare R2 location for 'Western Europe'

/// Stores information required to authenticate with a S3 bucket
pub struct BucketCredentials {
    pub credentials: Credentials,
    pub region: Region,
}

impl BucketCredentials {
    /// Creates a new bucket credential config
    ///
    /// # Arguments
    ///
    /// * `credentials` - The credentials required for login
    /// * `region` - The region to use
    ///
    /// # Returns
    /// Bucket credentials
    pub fn new(credentials: Credentials, region: Option<String>) -> Self {
        Self {
            credentials,
            region: Region::new(region.or_else(|| Some(DEFAULT_REGION.into())).unwrap()),
        }
    }
}
