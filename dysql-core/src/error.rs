use std::{fmt::{self, Display}, error::Error};
use serde::Serialize;

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

pub type DySqlResult<T> = Result<T, DySqlError>;

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
    pub message: Option<String>,
}

impl ErrorInner {
    pub fn new(kind: Kind, cause: Option<Box<dyn Error + Sync + Send>>, message: Option<String>) -> Self {
        Self {
            kind,
            cause,
            message
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
        let rst = if let Some(ref message) = self.0.message {
            let message = "sql error: ".to_owned() + message;
            fmt.write_str(&message)
        } else {
            match &self.0.kind {
                Kind::ParseSqlError => fmt.write_str("sql error: error parse sql"),
                Kind::PrepareStamentError => fmt.write_str("sql error: error preparement db statement"),
                Kind::BindParamterError => fmt.write_str("sql error: error bind db parameter"),
                Kind::TemplateNotFound => fmt.write_str("sql error: error sql template is not found"),
                Kind::TemplateParseError => fmt.write_str("sql error: error sql template parse"),
                Kind::QueryError => fmt.write_str("sql error: error db query"),
                Kind::ObjectMappingError => fmt.write_str("sql error: error object mapping"),
                Kind::ExtractSqlParamterError => fmt.write_str("sql error: error extract sql parameter"),
            }
        };
        
        let _= rst?;

        if let Some(ref cause) = self.0.cause {
            write!(fmt, ", cause: {}", cause)?;
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