use core::fmt;
use std::error::Error;

use serde::Serialize;

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