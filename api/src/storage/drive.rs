use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use super::driver::{StorageError, StorageResult};

macro_rules! try_write {
    ($f:expr, $i:expr) => {
        $f.write($i).map_err(|err| StorageError::from(err))?
    };
}

/// Implements save_file for the Drive type
pub(crate) async fn save_file(
    root: &PathBuf,
    id: &str,
    content_type: &str,
    bytes: Vec<u8>,
) -> StorageResult<()> {
    let mut file_path = root.clone();
    file_path.push(id);

    std::fs::create_dir_all(root).map_err(|err| StorageError::from(err))?;
    let mut file = File::create(file_path).map_err(|err| StorageError::from(err))?;
    try_write!(file, &[content_type.len() as u8]);
    try_write!(file, content_type.as_bytes());
    try_write!(file, &bytes);
    Ok(())
}

/// Implements get_file for the Drive type
pub(crate) async fn get_file(root: &PathBuf, id: &str) -> StorageResult<(Vec<u8>, String)> {
    let mut file_path = root.clone();
    file_path.push(id);

    let mut file = File::open(file_path).map_err(|_| StorageError::DriveLoadError)?;
    let content_type = read_content_type(&mut file)?;
    let bytes = read_file_bytes(&mut file)?;
    Ok((bytes, content_type))
}

/// Implements delete_file for the Drive type
pub(crate) async fn delete_file(root: &PathBuf, id: &str) -> StorageResult<()> {
    let mut file_path = root.clone();
    file_path.push(id);
    std::fs::remove_file(file_path).map_err(|_| StorageError::DriveDeleteError)
}

/// Reads the content type stored in the file
fn read_content_type(file: &mut File) -> StorageResult<String> {
    let mut ct_len = [0 as u8];
    file.read_exact(&mut ct_len)
        .map_err(|_| StorageError::DriveLoadError)?;
    let mut ct = vec![0 as u8; ct_len[0] as usize];
    file.read_exact(&mut ct)
        .map_err(|_| StorageError::DriveLoadError)?;
    String::from_utf8(ct).map_err(|_| StorageError::DriveLoadError)
}

/// Reads the file bytes stored in the file
fn read_file_bytes(file: &mut File) -> StorageResult<Vec<u8>> {
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|_| StorageError::DriveLoadError)?;
    Ok(bytes)
}
