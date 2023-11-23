use std::fmt::{Display, Formatter};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum SqlDialect {
    postgres,
    mysql,
    sqlite,
    mssql,
}

impl Display for SqlDialect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for SqlDialect {
    fn from(source: &str) -> Self {
        if source == SqlDialect::postgres.to_string() {
            SqlDialect::postgres
        } else if source == SqlDialect::mysql.to_string() {
            SqlDialect::mysql
        } else if source == SqlDialect::sqlite.to_string() {
            SqlDialect::sqlite
        } else if source == SqlDialect::sqlite.to_string() {
            SqlDialect::mssql
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