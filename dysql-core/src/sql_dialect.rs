use std::fmt::{Display, Formatter};

use once_cell::sync::OnceCell;

pub static DYSQL_CONFIG: OnceCell<DySqlConfig> = OnceCell::new();

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