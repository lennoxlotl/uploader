use rocket::{
    http::Status,
    response::{self, Responder},
    serde::json::Json,
    Request, Response,
};
use serde::Serialize;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Failed to upload image to storage bucket")]
    BucketConnectionError,
    #[error("Failed to convert image byte stream")]
    ImageConvertError,
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
            .status(Status::InternalServerError)
            .ok()
    }
}
