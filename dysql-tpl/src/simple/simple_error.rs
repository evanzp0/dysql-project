use std::{error::Error, fmt::{Display, self}};

///
pub type SimpleError = Box<dyn Error + Send + Sync>;

#[derive(Debug)]
pub struct SimpleInnerError(pub String);

impl Error for SimpleInnerError {
}

impl Display for SimpleInnerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "apply dto error: {:?}", self.0)
    }
}