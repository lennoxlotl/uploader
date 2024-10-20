use crate::endpoint::v1::create_v1_routes;
use endpoint::fairing::bucket::BucketFairing;

pub mod endpoint;
pub mod s3;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/api/v1/", create_v1_routes())
        .attach(BucketFairing::new())
        .launch()
        .await?;
    Ok(())
}
