use macros::PostgresRow;
use sqlx::Row;

/// Stores information about an uploaded file
#[derive(Debug, Clone, PostgresRow)]
pub struct FileEntity {
    pub id: String,
    pub bucket_id: String,
    pub secret: String,
    pub uploaded_at: i64,
    pub size: i64,
}
