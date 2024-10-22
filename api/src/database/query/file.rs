use std::time::{SystemTime, UNIX_EPOCH};

use super::{DbResult, PgTransaction};
use crate::database::file::FileEntity;

/// Finds a file by it's public id
pub async fn find_file_by_id(
    transaction: &mut PgTransaction<'_>,
    id: &String,
) -> DbResult<FileEntity> {
    sqlx::query_as::<_, FileEntity>(r"SELECT * FROM files WHERE id = $1")
        .bind(&id)
        .fetch_one(&mut **transaction)
        .await
}

/// Inserts a file into the database
pub async fn save_file(
    transaction: &mut PgTransaction<'_>,
    id: &String,
    bucket_id: &String,
    secret: &String,
    size: &i64,
) -> DbResult<()> {
    sqlx::query(
        r"INSERT INTO files (id, bucket_id, secret, uploaded_at, size) VALUES ($1, $2, $3, $4, $5)",
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

/// Deletes a file by it's secret id (given to uploader for deletion)
pub async fn delete_file_by_secret(
    transaction: &mut PgTransaction<'_>,
    secret: &String,
) -> DbResult<FileEntity> {
    sqlx::query_as::<_, FileEntity>(r"DELETE FROM files WHERE secret = $1 RETURNING *")
        .bind(&secret)
        .fetch_one(&mut **transaction)
        .await
}

/// Returns the time passed since the unix epoch in ms
fn since_epoch_in_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}
