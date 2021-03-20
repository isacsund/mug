use std::fmt;

use url::ParseError;

/// Error types
#[derive(Debug)]
pub enum Error {
    /// AUR error.
    Aur(String),
    /// There was an error parsing an URL.
    Url(ParseError),
    /// Reqwest returned an error.
    Reqwest(reqwest::Error),
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Error {
        Error::Url(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Aur(ref s) => write!(fmt, "{}", s),
            Error::Url(ref e) => write!(fmt, "{}", e),
            Error::Reqwest(ref e) => write!(fmt, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Aur(_) => None,
            Error::Url(ref e) => e.source(),
            Error::Reqwest(ref e) => e.source(),
        }
    }
}
