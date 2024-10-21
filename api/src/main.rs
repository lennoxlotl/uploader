use crate::endpoint::v1::create_v1_routes;
use endpoint::fairing::{bucket::BucketFairing, database::PostgresFairing};
use rocket::{fairing::AdHoc, routes};
use serde::{Deserialize, Serialize};

pub mod database;
pub mod endpoint;
pub mod s3;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    public_url: String,
    image_id_length: usize,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/api/v1/", create_v1_routes())
        .mount("/", routes![endpoint::show::show_image])
        .attach(AdHoc::config::<GlobalConfig>())
        .attach(BucketFairing::new())
        .attach(PostgresFairing::new())
        .launch()
        .await?;
    Ok(())
}
