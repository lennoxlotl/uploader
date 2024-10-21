use super::{
    fairing::{bucket::BucketGuard, database::PostgresDb},
    v1::{error::Error, UploaderResult},
};
use crate::{database::query::image::find_image_by_id, s3::bucket::BucketOperations};
use rocket::{
    get,
    http::ContentType,
    response::{self, Responder},
    Request, Response,
};
use std::{io::Cursor, str::FromStr};

pub struct ImageShowResponse {
    data: Vec<u8>,
    content_type: String,
}

impl ImageShowResponse {
    pub fn new(data: Vec<u8>, content_type: String) -> Self {
        Self { data, content_type }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ImageShowResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(ContentType::from_str(&self.content_type).unwrap_or(ContentType::default()))
            .streamed_body(Cursor::new(self.data))
            .ok()
    }
}

#[get("/<id>")]
pub async fn show_image(
    id: &str,
    database: PostgresDb,
    bucket: BucketGuard,
) -> UploaderResult<ImageShowResponse> {
    let mut transaction = database.begin().await.map_err(|_| Error::DatabaseError)?;
    let image = find_image_by_id(&mut transaction, &id.to_string())
        .await
        .unwrap();
    let data = bucket.get(&image.bucket_id).await.unwrap();
    let image_type = &data.content_type.ok_or(Error::ImageConvertError)?;
    let image_bytes = data
        .body
        .collect()
        .await
        .map_err(|_| Error::ImageConvertError)?
        .to_vec();
    Ok(ImageShowResponse::new(image_bytes, image_type.to_string()))
}
