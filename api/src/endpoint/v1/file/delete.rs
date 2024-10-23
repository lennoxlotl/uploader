use crate::{
    database::query::file::delete_file_by_secret,
    endpoint::{
        fairing::{database::PostgresDb, storage::StorageDriverGuard},
        v1::{error::Error, UploaderResult},
    },
};
use rocket::{delete, get};

// Also offer deletion using GET requests as some screenshotting / uploading tools do that unfortunately
#[get("/file/delete/<id>")]
pub async fn delete_get(
    id: &str,
    database: PostgresDb,
    storage: StorageDriverGuard,
) -> UploaderResult<()> {
    inner_delete(id, database, storage).await
}

#[delete("/file/delete/<id>")]
pub async fn delete(
    id: &str,
    database: PostgresDb,
    storage: StorageDriverGuard,
) -> UploaderResult<()> {
    inner_delete(id, database, storage).await
}

/// Deletes a file by its secret id, this prevents unauthorized third parties to
/// delete random file ids
async fn inner_delete(
    id: &str,
    database: PostgresDb,
    storage: StorageDriverGuard,
) -> UploaderResult<()> {
    let mut transaction = database.begin().await.map_err(|_| Error::DatabaseError)?;

    let file = delete_file_by_secret(&mut transaction, &id.to_string())
        .await
        .map_err(|_| Error::FileNotFoundError)?;
    storage
        .delete_file(&file.storage_id.as_str())
        .await
        .map_err(|err| Error::from(err))?;

    transaction
        .commit()
        .await
        .map_err(|_| Error::DatabaseError)?;
    Ok(())
}
