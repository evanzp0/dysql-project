//! Do dynamic-sql query through proc-macro
//! 
//! It bases on [**tokio-postgres**] and [**sqlx**] crate (default feature), you can switch them by setting the features. 
//! It uses [**Ramhorns**] the high performance template engine implementation of [**Mustache**]
//! 
//! ## Example (Sqlx)
//! 
//! ### main.rs
//! ```ignore
//! //...
//! 
//! # #[tokio::main]
//! async fn main() {
//!     let conn = connect_postgres_db().await;
//!     
//!     // fetch all
//!     let dto = UserDto{ id: None, name: None, age: Some(15) };
//!     let rst = fetch_all!(|&dto, &conn| -> User {
//!         r#"SELECT * FROM test_user 
//!         WHERE 1 = 1
//!           {{#name}}AND name = :name{{/name}}
//!           {{#age}}AND age > :age{{/age}}
//!         ORDER BY id"#
//!     }).unwrap();
//!     assert_eq!(
//!         vec![
//!             User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, 
//!             User { id: 3, name: Some("zhangsan".to_owned()), age: Some(35) }
//!         ], 
//!         rst
//!     );
//! 
//!     let rst = fetch_one!(...).unwrap();
//! 
//!     let rst = fetch_scalar!(...).unwrap();
//!     
//!     let affected_rows_num = execute!(...).unwrap();
//!     
//!     let insert_id = insert!(...).unwrap();
//! 
//!     sql!('sql_fragment_1', "select * from table1");
//!     let rst = fetch_one!(|...| sql_fragment_1 + "where age > 10").unwrap();
//! }
//! ```
//! 
//! ## Example (tokio-postgres)
//! Full example please see: [Dysql tokio-postgres example](https://github.com/evanzp0/dysql-project/tree/main/examples/with_tokio_postgres)
//! 
//! ## Example (sqlx)
//! Full example please see: [Dysql sqlx example](https://github.com/evanzp0/dysql-project/tree/main/examples/with_sqlx)
mod extract_sql;
mod pagenation;
mod sql_dialect;
mod error;
mod utils;

pub use extract_sql::*;
pub use pagenation::*;
pub use sql_dialect::*;
pub use error::*;
pub use utils::*;
