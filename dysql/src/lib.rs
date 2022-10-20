use std::fmt::{Display, Formatter};

mod to_sql;
mod extract_sql;

pub use to_sql::*;
pub use extract_sql::*;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum SqlDialect {
    postgres,
    mysql,
    sqlite,
    oracle,
    other,
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
        } else if source == SqlDialect::oracle.to_string() {
            SqlDialect::oracle
        } else if source == SqlDialect::sqlite.to_string() {
            SqlDialect::sqlite
        } else {
            SqlDialect::other
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