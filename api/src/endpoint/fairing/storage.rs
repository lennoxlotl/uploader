use std::ops::Deref;

use aws_credential_types::Credentials;
use rocket::{
    fairing::{self, Fairing, Info, Kind},
    request::{FromRequest, Outcome},
    Build, Request, Rocket,
};
use serde::Deserialize;

use crate::{
    s3::{bucket::Bucket, credentials::BucketCredentials},
    storage::driver::StorageDriver,
};

pub struct StorageDriverGuard(pub StorageDriver);

pub struct StorageDriverFairing;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageDriverType {
    ObjectStorage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageDriverFairingConfig {
    storage_type: StorageDriverType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ObjectStorageConfig {
    url: String,
    name: String,
    access_key: String,
    access_key_secret: String,
    region: Option<String>,
}

impl Deref for StorageDriverGuard {
    type Target = StorageDriver;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StorageDriverFairing {
    pub fn new() -> Self {
        Self {}
    }
}

#[rocket::async_trait]
impl Fairing for StorageDriverFairing {
    fn info(&self) -> Info {
        Info {
            name: "Storage Driver Fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let driver = match rocket
            .figment()
            .focus("storage")
            .extract::<StorageDriverFairingConfig>()
            .expect("Unable to load storage config, is it defined in Rocket.toml?")
            .storage_type
        {
            StorageDriverType::ObjectStorage => {
                let config: ObjectStorageConfig =
                    rocket.figment().focus("storage.object").extract().expect(
                        "Unable to load object storage config, is it defined in Rocket.toml?",
                    );
                StorageDriver::object(Bucket::new(
                    config.name,
                    config.url,
                    BucketCredentials::new(
                        Credentials::from_keys(config.access_key, config.access_key_secret, None),
                        config.region,
                    ),
                ))
            }
        };
        Ok(rocket.manage(driver))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StorageDriverGuard {
    type Error = crate::endpoint::v1::error::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(driver) = request.rocket().state::<StorageDriver>() {
            Outcome::Success(StorageDriverGuard(driver.clone()))
        } else {
            Outcome::Error((
                rocket::http::Status::InternalServerError,
                Self::Error::StorageUnavailableError,
            ))
        }
    }
}
