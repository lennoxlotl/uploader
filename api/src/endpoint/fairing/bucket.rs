use aws_credential_types::Credentials;
use rocket::{
    fairing::{self, Fairing, Info, Kind},
    request::{FromRequest, Outcome},
    Build, Request, Rocket,
};
use serde::Deserialize;
use std::ops::Deref;

use crate::s3::{bucket::Bucket, credentials::BucketCredentials};

/// Provides access to object storage buckets implementing the s3 api
#[derive(Debug, Clone)]
pub struct BucketGuard(pub Bucket);

pub struct BucketFairing;

#[derive(Debug, Clone, Deserialize)]
pub struct BucketFairingConfig {
    url: String,
    name: String,
    access_key: String,
    access_key_secret: String,
    region: Option<String>,
}

impl Deref for BucketGuard {
    type Target = Bucket;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BucketFairing {
    pub fn new() -> Self {
        Self {}
    }
}

#[rocket::async_trait]
impl Fairing for BucketFairing {
    fn info(&self) -> Info {
        Info {
            name: "Bucket Fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let fairing_config: BucketFairingConfig =
            rocket.figment().focus("bucket").extract().unwrap();
        Ok(rocket.manage(Bucket::new(
            fairing_config.name,
            fairing_config.url,
            BucketCredentials::new(
                Credentials::from_keys(
                    fairing_config.access_key,
                    fairing_config.access_key_secret,
                    None,
                ),
                fairing_config.region,
            ),
        )))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BucketGuard {
    type Error = crate::endpoint::v1::error::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(bucket) = request.rocket().state::<Bucket>() {
            Outcome::Success(BucketGuard(bucket.clone()))
        } else {
            Outcome::Error((
                rocket::http::Status::InternalServerError,
                Self::Error::BucketConnectionError,
            ))
        }
    }
}
