use std::fmt;

use url::ParseError;

/// Error types
#[derive(Debug)]
pub enum Error {
    /// Alpm returned an error.
    Alpm(alpm::Error),
    /// raur error.
    Raur(raur::Error),
    /// aur-fetch error
    Fetch(aur_fetch::Error),
    /// Configuration error.
    Config(String),
    /// There was an error parsing an URL.
    Url(ParseError),
}

impl From<alpm::Error> for Error {
    fn from(e: alpm::Error) -> Error {
        Error::Alpm(e)
    }
}

impl From<raur::Error> for Error {
    fn from(e: raur::Error) -> Error {
        Error::Raur(e)
    }
}

impl From<aur_fetch::Error> for Error {
    fn from(e: aur_fetch::Error) -> Error {
        Error::Fetch(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Error {
        Error::Url(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Alpm(ref s) => write!(fmt, "{}", s),
            Error::Raur(ref s) => write!(fmt, "{}", s),
            Error::Fetch(ref s) => write!(fmt, "{}", s),
            Error::Config(ref s) => write!(fmt, "{}", s),
            Error::Url(ref e) => write!(fmt, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Alpm(ref e) => e.source(),
            Error::Raur(ref e) => e.source(),
            Error::Fetch(ref e) => e.source(),
            Error::Config(_) => None,
            Error::Url(ref e) => e.source(),
        }
    }
}
