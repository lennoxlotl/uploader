use crate::{
    database::query::image::delete_image_by_secret,
    endpoint::{
        fairing::{bucket::BucketGuard, database::PostgresDb},
        v1::{error::Error, UploaderResult},
    },
    s3::bucket::BucketOperations,
};
use rocket::{delete, get};

// Also offer deletion using GET requests as some screenshotting tools do that unfortunately
#[get("/image/delete/<id>")]
pub async fn delete_get(id: &str, database: PostgresDb, bucket: BucketGuard) -> UploaderResult<()> {
    inner_delete(id, database, bucket).await
}

#[delete("/image/delete/<id>")]
pub async fn delete(id: &str, database: PostgresDb, bucket: BucketGuard) -> UploaderResult<()> {
    inner_delete(id, database, bucket).await
}

/// Deletes an image by its secret id, this prevents unauthorized third parties to
/// delete random image ids
async fn inner_delete(id: &str, database: PostgresDb, bucket: BucketGuard) -> UploaderResult<()> {
    let mut transaction = database.begin().await.map_err(|_| Error::DatabaseError)?;

    let image = delete_image_by_secret(&mut transaction, &id.to_string())
        .await
        .map_err(|_| Error::ImageNotFoundError)?;
    bucket
        .delete(&image.bucket_id.as_str())
        .await
        .map_err(|_| Error::BucketDeleteError)?;

    transaction
        .commit()
        .await
        .map_err(|_| Error::DatabaseError)?;
    Ok(())
}
