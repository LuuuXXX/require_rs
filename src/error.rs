use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    BadBinaryName,
    FailedFindBinary,
    FailedFindBinaryVersion,
    FailedFindExpectedBinaryVersion,
    InvalidBinaryVersionCommand,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadBinaryName => write!(f, "Bad binary name"),
            Error::FailedFindBinary => write!(f, "Failed to find binary"),
            Error::InvalidBinaryVersionCommand => write!(f, "Invalid binary version command"),
            Error::FailedFindBinaryVersion => write!(f, "Failed to find binary version"),
            Error::FailedFindExpectedBinaryVersion => write!(f, "Failed to find expected binary version"),
        }
    }
}