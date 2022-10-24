use std::{fmt::{Display, Formatter}, sync::RwLock, collections::HashMap};
use std::error::Error;
mod extract_sql;

pub use extract_sql::*;

use crypto::{md5::Md5, digest::Digest};
use once_cell::sync::OnceCell;
use ramhorns::{Template, Content};

pub static SQL_TEMPLATE_CACHE: OnceCell<RwLock<HashMap<String, Template>>> = OnceCell::new();
pub static DYSQL_CONFIG: OnceCell<DySqlConfig> = OnceCell::new();

pub type DySqlResult<T> = Result<T, Box<dyn Error>>;
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
fn get_sql_template_cache() -> &'static RwLock<HashMap<String, Template<'static>>> {
    let cache = SQL_TEMPLATE_CACHE.get_or_init(|| {
        RwLock::new(HashMap::new())
    });

    cache
}

pub fn get_sql_template(template_id: &str) -> Option<*const Template<'static>> {
    let cache = get_sql_template_cache();

    let template_stub = cache.read().unwrap();
    let template = template_stub.get(template_id);

    if let Some(tmpl) = template {
        // println!("get template from cache: {}", template_id);
        return Some(tmpl as *const Template)
    }

    None
}

pub fn put_sql_template(template_id: &str, sql: &'static str) -> DySqlResult<*const Template<'static>> {
    // println!("put template to cache: {}", template_id);
    let cache = get_sql_template_cache();

    let template = Template::new(sql)?;
    cache.write().unwrap().insert(template_id.to_string(), template);

    let template = get_sql_template(template_id);

    if let Some(tmpl) = template {
        return Ok(tmpl as *const Template)
    }

    Err(Box::new(DySqlError::new(&format!("Template({}) is not find.", template_id))))
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

#[derive(Debug, PartialEq)]
pub struct DySqlError {
    pub msg: String
}

impl DySqlError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl Display for DySqlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for DySqlError {
    fn cause(&self) -> Option<&dyn Error> {
       None
    }
}

#[derive(Debug)]
pub enum QueryType {
    FetchAll,
    FetchOne,
    FetchScalar,
    Execute,
    Insert,
}

#[derive(Content)]
pub struct EmptyDto {
    pub _empty: u8
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