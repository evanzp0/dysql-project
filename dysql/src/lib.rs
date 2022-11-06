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
//! }
//! ```
//! 
//! ## Example (tokio-postgres)
//! Full example please see: [Dysql tokio-postgres example](https://github.com/evanzp0/dysql-project/tree/main/examples/with_tokio_postgres)
//! 
//! ## Example (sqlx)
//! Full example please see: [Dysql sqlx example](https://github.com/evanzp0/dysql-project/tree/main/examples/with_sqlx)
mod extract_sql;

pub use extract_sql::*;

use core::fmt;
use std::{fmt::{Display, Formatter}, sync::{RwLock, Arc}, collections::HashMap};
use std::error::Error;

use serde::Serialize;
use crypto::{md5::Md5, digest::Digest};
use once_cell::sync::OnceCell;
use ramhorns::{Template, Content};

// pub static DEFAULT_ERROR_MSG: &str = "Error occurs when extracting sql parameters.";
pub static SQL_TEMPLATE_CACHE: OnceCell<RwLock<HashMap<String, Arc<Template>>>> = OnceCell::new();
pub static DYSQL_CONFIG: OnceCell<DySqlConfig> = OnceCell::new();

pub type DySqlResult<T> = Result<T, DySqlError>;
pub struct DySqlConfig {
    pub  dialect: SqlDialect
}

impl DySqlConfig {
    pub fn new() -> Self {
        Self {
            dialect: SqlDialect::postgres
        }
    }
}

pub fn get_dysql_config() -> &'static DySqlConfig {
    let cfg = DYSQL_CONFIG.get_or_init(|| {
        DySqlConfig::new()
    });

    cfg
}

#[allow(dead_code)]
fn get_sql_template_cache() -> &'static RwLock<HashMap<String, Arc<Template<'static>>>> {
    let cache = SQL_TEMPLATE_CACHE.get_or_init(|| {
        RwLock::new(HashMap::new())
    });

    cache
}

pub fn get_sql_template(template_id: &str) -> Option<Arc<Template<'static>>> {
    let cache = get_sql_template_cache();

    let template_stub = cache.read().unwrap();
    let template = template_stub.get(template_id);

    if let Some(tmpl) = template {
        // println!("get template from cache: {}", template_id);
        return Some(tmpl.clone())
    }

    None
}

pub fn put_sql_template(template_id: &str, sql: &'static str) -> DySqlResult<Arc<Template<'static>>> {
    // println!("put template to cache: {}", template_id);
    let cache = get_sql_template_cache();

    let template = Template::new(sql).map_err(|e| {
        DySqlError(ErrorInner::new(Kind::TemplateParseError, Some(Box::new(e))))
    })?;
    cache.write().unwrap().insert(template_id.to_string(), Arc::new(template));

    let template = get_sql_template(template_id);

    if let Some(tmpl) = template {
        return Ok(tmpl.clone())
    }

    Err(DySqlError(ErrorInner::new(Kind::TemplateNotFound, None)))

}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum SqlDialect {
    postgres,
    mysql,
    sqlite,
}

impl Display for SqlDialect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<String> for SqlDialect {
    fn from(source: String) -> Self {
        if source == SqlDialect::postgres.to_string() {
            SqlDialect::postgres
        } else if source == SqlDialect::mysql.to_string() {
            SqlDialect::mysql
        } else if source == SqlDialect::sqlite.to_string() {
            SqlDialect::sqlite
        } else {
            panic!("{} dialect is not support", source);
        }
    }
}

impl PartialEq<String> for SqlDialect {
    fn eq(&self, other: &String) -> bool {
        *other == self.to_string()
    }
}

pub fn md5<S:Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}

#[derive(Debug)]
pub enum QueryType {
    FetchAll,
    FetchOne,
    FetchScalar,
    Execute,
    Insert,
}


#[derive(Debug, Serialize, PartialEq)]
pub enum Kind {
    ParseSqlError,
    PrepareStamentError,
    BindParamterError,
    TemplateNotFound,
    TemplateParseError,
    ExtractSqlParamterError,
    QueryError,
    ObjectMappingError,
}

#[derive(Debug, Serialize)]
pub struct ErrorInner {
    pub kind: Kind,
    #[serde(skip_serializing)]
    pub cause: Option<Box<dyn Error + Sync + Send>>,
}

impl ErrorInner {
    pub fn new(kind: Kind, cause: Option<Box<dyn Error + Sync + Send>>) -> Self {
        Self {
            kind,
            cause
        }
    }
}

#[derive(Serialize)]
pub struct DySqlError(pub ErrorInner);

impl fmt::Debug for DySqlError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Error")
            .field("kind", &self.0.kind)
            .field("cause", &self.0.cause)
            .finish()
    }
}

impl fmt::Display for DySqlError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0.kind {
            Kind::ParseSqlError => fmt.write_str("error parse sql")?,
            Kind::PrepareStamentError => fmt.write_str("error preparement db statement")?,
            Kind::BindParamterError => fmt.write_str("error bind db parameter")?,
            Kind::TemplateNotFound => fmt.write_str("error sql template is not found")?,
            Kind::TemplateParseError => fmt.write_str("error sql template parse")?,
            Kind::ExtractSqlParamterError => fmt.write_str("error extract sql parameter")?,
            Kind::QueryError => fmt.write_str("error db query")?,
            Kind::ObjectMappingError => fmt.write_str("error object mapping")?,
        };
        if let Some(ref cause) = self.0.cause {
            write!(fmt, ": {}", cause)?;
        }
        Ok(())
    }
}

impl Error for DySqlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.cause.as_ref().map(|e| &**e as _)
    }
}

unsafe impl Send for DySqlError {}
unsafe impl Sync for DySqlError {}

#[allow(unused)]
#[derive(Content)]
pub struct Value<T> {
    pub value: T
}

impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let s = SqlDialect::postgres.to_string();
        assert_eq!("postgres", s);
    }
}