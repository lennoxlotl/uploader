use aws_sdk_s3::primitives::ByteStream;
use rocket::{
    tokio::io::{AsyncBufRead, AsyncReadExt},
    Route,
};

pub mod error;
pub mod file;

pub type UploaderResult<T> = std::result::Result<T, error::Error>;

/// Creates all /api/v1/ routes for initialization
pub fn create_v1_routes() -> Vec<Route> {
    rocket::routes![
        file::upload::upload,
        file::delete::delete,
        file::delete::delete_get
    ]
}

/// Converts a tokio buffer (from form data) to a with the s3 api usable bytestream
///
/// # Arguments
///
/// * `stream` - The mutable byte buf
///
/// # Returns
///
/// The converted stream
pub(crate) async fn convert_to_byte_stream<T>(stream: &mut T) -> Result<ByteStream, error::Error>
where
    T: AsyncBufRead + AsyncReadExt + Unpin,
{
    let mut bytes: Vec<u8> = Vec::new();
    stream
        .read_to_end(&mut bytes)
        .await
        .map_err(|_| error::Error::FileConvertError)?;
    return Ok(ByteStream::from(bytes));
}
