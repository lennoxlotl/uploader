use super::{
    fairing::{bucket::BucketGuard, database::PostgresDb},
    v1::{error::Error, UploaderResult},
};
use crate::{database::query::image::find_image_by_id, s3::bucket::BucketOperations, GlobalConfig};
use rocket::{
    get,
    http::ContentType,
    response::{self, Responder},
    Request, Response, State,
};
use std::{io::Cursor, str::FromStr};

pub struct ImageShowResponse {
    data: Vec<u8>,
    content_type: String,
    cache_time: usize,
}

impl ImageShowResponse {
    pub fn new(data: Vec<u8>, content_type: String, cache_time: usize) -> Self {
        Self {
            data,
            content_type,
            cache_time,
        }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ImageShowResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(ContentType::from_str(&self.content_type).unwrap_or(ContentType::default()))
            .raw_header(
                "Cache-Control",
                if self.cache_time > 0 {
                    format!("max-age={}", self.cache_time)
                } else {
                    "no-cache".into()
                },
            )
            .streamed_body(Cursor::new(self.data))
            .ok()
    }
}

// TODO: come up with something better
#[get("/")]
pub async fn index() -> &'static str {
    "hi :wave:"
}

#[get("/<id>")]
pub async fn show_image(
    id: &str,
    database: PostgresDb,
    bucket: BucketGuard,
    config: &State<GlobalConfig>,
) -> UploaderResult<ImageShowResponse> {
    let mut transaction = database.begin().await.map_err(|_| Error::DatabaseError)?;
    let image = find_image_by_id(&mut transaction, &id.to_string())
        .await
        .map_err(|_| Error::ImageNotFoundError)?;
    let data = bucket.get(&image.bucket_id).await.unwrap();
    let image_type = &data.content_type.ok_or(Error::ImageConvertError)?;
    let image_bytes = data
        .body
        .collect()
        .await
        .map_err(|_| Error::ImageConvertError)?
        .to_vec();
    Ok(ImageShowResponse::new(
        image_bytes,
        image_type.to_string(),
        config.cache_length.unwrap_or(0),
    ))
}
