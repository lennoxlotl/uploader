use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use rocket::{post, State};

use crate::endpoint::fairing::bucket::BucketGuard;
use crate::endpoint::v1::error::Error;
use crate::endpoint::v1::{convert_to_byte_stream, UploaderResult};
use crate::s3::bucket::BucketOperations;
use crate::GlobalConfig;

#[derive(FromForm)]
pub struct ImageData<'r> {
    image: TempFile<'r>,
}

#[post("/image/upload", data = "<image_data>")]
pub async fn upload(
    image_data: Form<ImageData<'_>>,
    bucket: BucketGuard,
    config: &State<GlobalConfig>,
) -> UploaderResult<&'static str> {
    bucket
        .put(
            generate_image_id(config.image_id_length).as_str(),
            convert_to_byte_stream(
                &mut image_data
                    .image
                    .open()
                    .await
                    .map_err(|_| Error::ImageConvertError)?,
            )
            .await?,
            Some("image/png"),
        )
        .await
        .map_err(|_| Error::BucketConnectionError)?;
    Ok("hi")
}

/// Generates a randomized image id
///
/// # Arguments
/// * `size` - The amount of characters to generate
fn generate_image_id(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
