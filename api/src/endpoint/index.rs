use super::{
    fairing::{database::PostgresDb, storage::StorageDriverGuard},
    v1::{error::Error, UploaderResult},
    SuccessReporter,
};
use crate::{database::query::file::find_file_by_id, GlobalConfig};
use build_info::BuildInfo;
use rocket::{
    get,
    http::ContentType,
    response::{self, Responder},
    serde::json::Json,
    Request, Response, State,
};
use serde::Serialize;
use std::{io::Cursor, str::FromStr};

pub struct FileShowResponse {
    data: Vec<u8>,
    content_type: String,
    cache_time: usize,
}

#[derive(Debug, Serialize)]
pub struct ServerInfoResponse {
    #[serde(flatten)]
    success: SuccessReporter,
    version: String,
}

impl ServerInfoResponse {
    pub fn new(version: String) -> Self {
        Self {
            success: SuccessReporter::new(true),
            version,
        }
    }
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

#[get("/")]
pub async fn index() -> Json<ServerInfoResponse> {
    let info: &BuildInfo = build_info();
    Json(ServerInfoResponse::new(info.crate_info.version.to_string()))
}

#[get("/<id>")]
pub async fn show_file(
    id: &str,
    database: PostgresDb,
    storage: StorageDriverGuard,
    config: &State<GlobalConfig>,
) -> UploaderResult<FileShowResponse> {
    let mut transaction = database.begin().await.map_err(|_| Error::DatabaseError)?;
    let file = find_file_by_id(&mut transaction, &id.to_string())
        .await
        .map_err(|_| Error::FileNotFoundError)?;
    let (data, content_type) = storage.get_file(&file.storage_id).await.unwrap();
    Ok(FileShowResponse::new(
        data,
        content_type,
        config.cache_length.unwrap_or(0),
    ))
}

build_info::build_info!(fn build_info);
