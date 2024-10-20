use crate::endpoint::v1::create_v1_routes;
use endpoint::fairing::bucket::BucketFairing;
use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};

pub mod database;
pub mod endpoint;
pub mod s3;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    image_id_length: usize,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/api/v1/", create_v1_routes())
        .attach(AdHoc::config::<GlobalConfig>())
        .attach(BucketFairing::new())
        .launch()
        .await?;
    Ok(())
}
