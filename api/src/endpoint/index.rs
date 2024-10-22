use super::{
    fairing::{bucket::BucketGuard, database::PostgresDb},
    v1::{error::Error, UploaderResult},
};
use crate::{database::query::file::find_file_by_id, s3::bucket::BucketOperations, GlobalConfig};
use rocket::{
    get,
    http::ContentType,
    response::{self, Responder},
    Request, Response, State,
};
use std::{io::Cursor, str::FromStr};

pub struct FileShowResponse {
    data: Vec<u8>,
    content_type: String,
    cache_time: usize,
}

impl FileShowResponse {
    pub fn new(data: Vec<u8>, content_type: String, cache_time: usize) -> Self {
        Self {
            data,
            content_type,
            cache_time,
        }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for FileShowResponse {
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
pub async fn show_file(
    id: &str,
    database: PostgresDb,
    bucket: BucketGuard,
    config: &State<GlobalConfig>,
) -> UploaderResult<FileShowResponse> {
    let mut transaction = database.begin().await.map_err(|_| Error::DatabaseError)?;
    let file = find_file_by_id(&mut transaction, &id.to_string())
        .await
        .map_err(|_| Error::FileNotFoundError)?;
    let data = bucket.get(&file.bucket_id).await.unwrap();
    let file_type = &data.content_type.ok_or(Error::FileConvertError)?;
    let file_bytes = data
        .body
        .collect()
        .await
        .map_err(|_| Error::FileConvertError)?
        .to_vec();
    Ok(FileShowResponse::new(
        file_bytes,
        file_type.to_string(),
        config.cache_length.unwrap_or(0),
    ))
}
