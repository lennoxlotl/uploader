use macros::UploaderError;
use rocket::{
    http::Status,
    response::{self, Responder},
    serde::json::Json,
    Request, Response,
};
use serde::Serialize;

/// Stores attributes about an error
pub struct ErrorAttributes {
    pub status_code: u16,
}

#[derive(Debug, Clone, thiserror::Error, UploaderError)]
pub enum Error {
    #[error("The image does not exist")]
    #[uploader(status_code = 404)]
    ImageNotFoundError,
    #[error("Failed to upload image to storage bucket")]
    #[uploader(status_code = 500)]
    BucketConnectionError,
    #[error("Failed to convert image byte stream")]
    #[uploader(status_code = 500)]
    ImageConvertError,
    #[error("Failed to execute database operation")]
    #[uploader(status_code = 500)]
    DatabaseError,
}

#[derive(Debug, Serialize)]
pub struct RocketErrorResponse {
    message: String,
}

impl RocketErrorResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .merge(Json(RocketErrorResponse::new(self.to_string())).respond_to(request)?)
            .status(
                Status::from_code(self.error_attr().status_code)
                    .unwrap_or(Status::InternalServerError),
            )
            .ok()
    }
}
