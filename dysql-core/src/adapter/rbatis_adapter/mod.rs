#[cfg(all(feature = "rbs", feature = "rbatis"))]
mod common;

#[cfg(all(feature = "rbs", feature = "rbatis"))]
pub use common::*;

#[cfg(feature = "rbatis-sqlite")]
mod sqlite_adapter;
#[cfg(feature = "rbatis-sqlite")]
pub use sqlite_adapter::*;

#[cfg(all(feature = "rbs", feature = "rbatis"))]
#[macro_use] pub mod adapter_macro;