use std::{fmt::{self, Display}, error::Error};

#[derive(Debug)]
pub struct ParseSqlError(pub String);

impl Display for ParseSqlError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "parse sql error: {}", self.0)
    }
}

impl Error for ParseSqlError {
    
}

pub type ParseSqlResult<T> = Result<T, ParseSqlError>;