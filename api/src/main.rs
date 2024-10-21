use crate::endpoint::v1::create_v1_routes;
use endpoint::fairing::{bucket::BucketFairing, database::PostgresFairing};
use rocket::{fairing::AdHoc, routes};
use serde::{Deserialize, Serialize};

pub mod database;
pub mod endpoint;
pub mod s3;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    // Public server url
    public_url: String,
    // Length of the image id used for "shwoing" the image
    image_id_length: usize,
    // Defines a Cache-Control header, time is in seconds
    cache_length: Option<usize>,
    // If not empty requires an authentication header containing this key for uploads
    auth_key: Option<String>,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/api/v1/", create_v1_routes())
        .mount(
            "/",
            routes![endpoint::index::index, endpoint::index::show_image],
        )
        .attach(AdHoc::config::<GlobalConfig>())
        .attach(BucketFairing::new())
        .attach(PostgresFairing::new())
        .launch()
        .await?;
    Ok(())
}
