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

/// Converts a tokio buffer (from form data) to vector of bytes
///
/// # Arguments
///
/// * `stream` - The mutable byte buf
///
/// # Returns
///
/// The converted vector of bytes
pub(crate) async fn convert_to_bytes<T>(stream: &mut T) -> Result<Vec<u8>, error::Error>
where
    T: AsyncBufRead + AsyncReadExt + Unpin,
{
    let mut bytes: Vec<u8> = Vec::new();
    stream
        .read_to_end(&mut bytes)
        .await
        .map_err(|_| error::Error::FileConvertError)?;
    return Ok(bytes);
}
