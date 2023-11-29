#![allow(unused, ambiguous_glob_reexports)]

#[cfg(feature = "sqlx")]
mod sqlx_adapter;

#[cfg(feature = "sqlx")]
pub use sqlx_adapter::*;

#[cfg(feature = "tokio-postgres")]
mod tokio_pg_adapter;

#[cfg(feature = "tokio-postgres")]
pub use tokio_pg_adapter::*;

#[cfg(all(feature = "rbs", feature = "rbatis"))]
mod rbatis_adapter;

#[cfg(all(feature = "rbs", feature = "rbatis"))]
pub use rbatis_adapter::*;
