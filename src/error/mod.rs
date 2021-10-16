use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "{}", e),
            Error::Custom(e) => write!(f, "{}", e)
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl std::error::Error for Error {}

pub fn new_error<S: AsRef<str>>(text: S) -> Error {
    Error::Custom(text.as_ref().to_string())
}