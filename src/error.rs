use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    FailedFindBinary,
    FailedGetBinaryPath,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FailedFindBinary => write!(f, "Failed to find binary"),
            Error::FailedGetBinaryPath => write!(f, "Failed to get binary path"),
        }
    }
}