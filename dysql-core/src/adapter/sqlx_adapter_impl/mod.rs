#[cfg(feature = "sqlx-postgres")]
mod postgres_adapter;

#[cfg(feature = "sqlx-mysql")]
mod mysql_adapter;

#[cfg(feature = "sqlx-sqlite")]
mod sqlite_adapter;
