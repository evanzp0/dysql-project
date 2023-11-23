#[cfg(feature = "sqlx")]
#[macro_use] mod common;

#[cfg(feature = "sqlx")]
pub use common::*;

#[cfg(feature = "sqlx-postgres")]
mod postgres_adapter;

#[cfg(feature = "sqlx-mysql")]
mod mysql_adapter;

#[cfg(feature = "sqlx-sqlite")]
mod sqlite_adapter;

#[cfg(feature = "sqlx")]
#[macro_use] pub mod adapter_macro;
