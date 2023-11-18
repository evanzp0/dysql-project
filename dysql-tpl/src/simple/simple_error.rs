use std::{error::Error, fmt::{Display, self}};


#[derive(Debug)]
pub struct SimpleError(pub String);

impl Error for SimpleError {
}

impl Display for SimpleError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "apply dto error: {:?}", self.0)
    }
}