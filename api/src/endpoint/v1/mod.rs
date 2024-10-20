use aws_sdk_s3::primitives::ByteStream;
use rocket::{
    tokio::io::{AsyncBufRead, AsyncReadExt},
    Route,
};

pub mod error;
pub mod image;

pub type UploaderResult<T> = std::result::Result<T, error::Error>;

/// Creates all /api/v1/ routes for initialization
pub fn create_v1_routes() -> Vec<Route> {
    rocket::routes![image::upload::upload]
}

pub(crate) async fn convert_to_byte_stream<T>(stream: &mut T) -> Result<ByteStream, error::Error>
where
    T: AsyncBufRead + AsyncReadExt + Unpin,
{
    let mut bytes: Vec<u8> = Vec::new();
    stream
        .read_to_end(&mut bytes)
        .await
        .map_err(|_| error::Error::ImageConvertError)?;
    return Ok(ByteStream::from(bytes));
}
