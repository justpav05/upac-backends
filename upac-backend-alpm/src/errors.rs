use std::fmt;
use std::io;
use std::result;

#[derive(Debug)]
pub enum BackendError {
    UnsupportedFormat(String),
    InvalidPackage(String),
    Io(io::Error),
}

pub type Result<T> = result::Result<T, BackendError>;

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackendError::UnsupportedFormat(string) => write!(f, "Unsupported format: {string}"),
            BackendError::InvalidPackage(string)    => write!(f, "Invalid package: {string}"),
            BackendError::Io(err)                => write!(f, "IO error: {err}"),
        }
    }
}

// Чтобы ? работал для std::io::Error автоматически
impl From<io::Error> for BackendError {
    fn from(err: io::Error) -> Self {
        BackendError::Io(err)
    }
}
