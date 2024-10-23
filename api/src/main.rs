use crate::endpoint::v1::create_v1_routes;
use endpoint::fairing::{database::PostgresFairing, storage::StorageDriverFairing};
use rocket::{fairing::AdHoc, routes};
use serde::{Deserialize, Serialize};

pub mod database;
pub mod endpoint;
pub mod s3;
pub mod storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    // Public server url
    public_url: String,
    // Length of the file id used for "shwoing" the file
    file_id_length: usize,
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
            routes![endpoint::index::index, endpoint::index::show_file],
        )
        .attach(AdHoc::config::<GlobalConfig>())
        .attach(StorageDriverFairing::new())
        .attach(PostgresFairing::new())
        .launch()
        .await?;
    Ok(())
}
