#[cfg(all(feature = "rbs", feature = "rbatis"))]
mod common;

#[cfg(all(feature = "rbs", feature = "rbatis"))]
pub use common::*;

#[cfg(all(feature = "rbs", feature = "rbatis"))]
#[macro_use] pub mod adapter_macro;

#[cfg(feature = "rbatis-sqlite")]
mod sqlite_adapter;

#[cfg(feature = "rbatis-sqlite")]
pub use sqlite_adapter::*;

#[cfg(feature = "rbatis-pg")]
mod postgres_adapter;

#[cfg(feature = "rbatis-pg")]
pub use postgres_adapter::*;

#[cfg(feature = "rbatis-mysql")]
mod mysql_adapter;

#[cfg(feature = "rbatis-mysql")]
pub use mysql_adapter::*;