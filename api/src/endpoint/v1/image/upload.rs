use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::{post, Responder, State};
use serde::Serialize;
use uuid::Uuid;

use crate::database::query::image::save_image;
use crate::endpoint::fairing::bucket::BucketGuard;
use crate::endpoint::fairing::database::PostgresDb;
use crate::endpoint::v1::error::Error;
use crate::endpoint::v1::{convert_to_byte_stream, UploaderResult};
use crate::s3::bucket::BucketOperations;
use crate::GlobalConfig;

#[derive(FromForm)]
pub struct ImageData<'r> {
    image: TempFile<'r>,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct UploadResponse {
    pub inner: Json<UploadResponseData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UploadResponseData {
    url: String,
    deletion_url: String,
}

impl UploadResponse {
    pub fn new(url: String, deletion_url: String) -> Self {
        Self {
            inner: Json(UploadResponseData { url, deletion_url }),
        }
    }
}

#[post("/image/upload", data = "<image_data>")]
pub async fn upload(
    image_data: Form<ImageData<'_>>,
    bucket: BucketGuard,
    database: PostgresDb,
    config: &State<GlobalConfig>,
) -> UploaderResult<UploadResponse> {
    let mut transaction = database.begin().await.map_err(|_| Error::DatabaseError)?;
    let bucket_id = Uuid::new_v4().to_string().replace("-", "");
    let secret = Uuid::new_v4().to_string().replace("-", "");
    let id = generate_image_id(config.image_id_length);

    // As we use transactions, if the image upload fails the image will be dropped
    save_image(
        &mut transaction,
        &id,
        &bucket_id.to_string(),
        &secret,
        &(image_data.image.len() as i64),
    )
    .await
    .map_err(|_| Error::DatabaseError)?;
    bucket
        .put(
            &bucket_id,
            convert_to_byte_stream(
                &mut image_data
                    .image
                    .open()
                    .await
                    .map_err(|_| Error::ImageConvertError)?,
            )
            .await?,
            Some(
                &image_data
                    .image
                    .content_type()
                    .unwrap_or(&ContentType::default())
                    .to_string(),
            ),
        )
        .await
        .map_err(|_| Error::BucketConnectionError)?;

    transaction
        .commit()
        .await
        .map_err(|_| Error::DatabaseError)?;
    // TODO: Make api url configurable
    Ok(UploadResponse::new(
        format!("http://localhost:8000/{}", &id),
        format!("http://localhost:8000/api/v1/image/delete/{}", &secret),
    ))
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
