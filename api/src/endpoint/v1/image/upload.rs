use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use rocket::post;
use rocket::tokio::io::AsyncReadExt;

use crate::endpoint::fairing::bucket::BucketGuard;
use crate::endpoint::v1::error::Error;
use crate::endpoint::v1::{convert_to_byte_stream, UploaderResult};
use crate::s3::bucket::BucketOperations;

#[derive(FromForm)]
pub struct ImageData<'r> {
    image: TempFile<'r>,
}

#[post("/image/upload", data = "<image_data>")]
pub async fn upload(
    image_data: Form<ImageData<'_>>,
    bucket: BucketGuard,
) -> UploaderResult<&'static str> {
    let mut stream = image_data.image.open().await.unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    stream.read_to_end(&mut bytes).await.unwrap();
    bucket
        .put(
            "test-key",
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
        .unwrap();
    Ok("hi")
}
