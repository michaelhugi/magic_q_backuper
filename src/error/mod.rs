use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    SerdeJsonError(serde_json::Error),
    Custom(Vec<String>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "{}", e),
            Error::SerdeJsonError(e) => write!(f, "{}", e),
            Error::Custom(e) => {
                let mut err = "".to_string();
                for e in e.iter() {
                    err = format!("{}\n{}", err, e)
                }
                write!(f, "{}", err)
            }
        }
    }
}

impl Error {
    pub fn texts(self) -> Vec<String> {
        match self {
            Error::IOError(e) => vec![e.to_string()],
            Error::SerdeJsonError(e) => vec![e.to_string()],
            Error::Custom(e) => e
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJsonError(e)
    }
}

impl std::error::Error for Error {}

pub fn new_error<S: AsRef<str>>(text: Vec<S>) -> Error {
    let mut v = Vec::new();
    for text in text.into_iter() {
        v.push(text.as_ref().to_string());
    }
    Error::Custom(v)
}

pub fn new_error_s<S: AsRef<str>>(text: S) -> Error {
    Error::Custom(vec![text.as_ref().to_string()])
}

pub fn new_error_j<S: AsRef<str>>(text: S, cause: Error) -> Error {
    let mut v = Vec::new();
    v.push(text.as_ref().to_string());
    for t in cause.texts().into_iter() {
        v.push(t);
    }
    Error::Custom(v)
}