use std::convert::Infallible;

use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use rocket::http::ContentType;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::serde::json::Json;
use rocket::{post, Request, Responder, State};
use serde::Serialize;
use uuid::Uuid;

use crate::database::query::file::save_file;
use crate::endpoint::fairing::bucket::BucketGuard;
use crate::endpoint::fairing::database::PostgresDb;
use crate::endpoint::v1::error::Error;
use crate::endpoint::v1::{convert_to_byte_stream, UploaderResult};
use crate::s3::bucket::BucketOperations;
use crate::GlobalConfig;

#[derive(FromForm)]
pub struct FileData<'r> {
    file: TempFile<'r>,
}

pub struct AuthToken(String);

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => Outcome::Success(AuthToken(token.to_string())),
            None => Outcome::Success(AuthToken("".into())),
        }
    }
}

#[post("/file/upload", data = "<file_data>")]
pub async fn upload(
    file_data: Form<FileData<'_>>,
    bucket: BucketGuard,
    database: PostgresDb,
    config: &State<GlobalConfig>,
    token: AuthToken,
) -> UploaderResult<UploadResponse> {
    let mut transaction = database.begin().await.map_err(|_| Error::DatabaseError)?;
    let bucket_id = Uuid::new_v4().to_string().replace("-", "");
    let secret = Uuid::new_v4().to_string().replace("-", "");
    let id = generate_file_id(config.file_id_length);

    if let Some(auth_key) = &config.auth_key {
        if auth_key != &token.0 {
            return Err(Error::Unauthorized);
        }
    }

    // As we use transactions, if the image upload fails the image will be dropped
    save_file(
        &mut transaction,
        &id,
        &bucket_id.to_string(),
        &secret,
        &(file_data.file.len() as i64),
    )
    .await
    .map_err(|_| Error::DatabaseError)?;
    bucket
        .put(
            &bucket_id,
            convert_to_byte_stream(
                &mut file_data
                    .file
                    .open()
                    .await
                    .map_err(|_| Error::FileConvertError)?,
            )
            .await?,
            Some(
                &file_data
                    .file
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
        format!("{}/{}", config.public_url, &id),
        format!("{}/api/v1/image/delete/{}", config.public_url, &secret),
    ))
}

/// Generates a randomized file id
///
/// # Arguments
/// * `size` - The amount of characters to generate
fn generate_file_id(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
