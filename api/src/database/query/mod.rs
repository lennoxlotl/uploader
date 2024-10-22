pub mod file;

pub type PgTransaction<'a> = sqlx::Transaction<'a, sqlx::Postgres>;
pub type DbResult<T> = std::result::Result<T, sqlx::Error>;
