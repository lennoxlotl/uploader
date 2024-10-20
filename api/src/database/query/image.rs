use std::time::{SystemTime, UNIX_EPOCH};

use super::{DbResult, PgTransaction};
use crate::database::image::ImageEntity;

/// Finds an image by it's public id
pub async fn find_image_by_id(
    transaction: &mut PgTransaction<'_>,
    id: &String,
) -> DbResult<ImageEntity> {
    sqlx::query_as::<_, ImageEntity>(r"SELECT * FROM images WHERE id = $1")
        .bind(&id)
        .fetch_one(&mut **transaction)
        .await
}

/// Finds an image by it's secret id (given to uploader for deletion)
pub async fn find_image_by_secret(
    transaction: &mut PgTransaction<'_>,
    secret: &String,
) -> DbResult<ImageEntity> {
    sqlx::query_as::<_, ImageEntity>(r"SELECT * FROM images WHERE secret = $1")
        .bind(&secret)
        .fetch_one(&mut **transaction)
        .await
}

/// Inserts an image into the database
pub async fn save_image(
    transaction: &mut PgTransaction<'_>,
    id: &String,
    bucket_id: &String,
    secret: &String,
    size: &i64,
) -> DbResult<()> {
    sqlx::query(
        r"INSERT INTO images (id, bucket_id, secret, uploaded_at, size) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&id)
    .bind(&bucket_id)
    .bind(&secret)
    .bind(&since_epoch_in_ms())
    .bind(&size)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
}

/// Deletes an image by it's secret id (given to uploader for deletion)
pub async fn delete_image_by_secret(
    transaction: &mut PgTransaction<'_>,
    secret: &String,
) -> DbResult<()> {
    sqlx::query(r"DELETE FROM images WHERE secret = $1")
        .bind(&secret)
        .execute(&mut **transaction)
        .await
        .map(|_| ())
}

/// Returns the time passed since the unix epoch in ms
fn since_epoch_in_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}
