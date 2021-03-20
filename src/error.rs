use std::fmt;

/// Error types
#[derive(Debug)]
pub enum Error {
    /// AUR error.
    Aur(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Aur(ref s) => write!(fmt, "{}", s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Aur(_) => None,
        }
    }
}
