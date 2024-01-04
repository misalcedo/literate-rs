use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::path::StripPrefixError;

#[derive(Debug)]
pub enum LiterateError {
    IO(io::Error),
    #[cfg(feature = "walk")]
    Walk(walkdir::Error),
    #[cfg(feature = "walk")]
    Prefix(StripPrefixError),
    Unknown,
}

impl From<io::Error> for LiterateError {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<walkdir::Error> for LiterateError {
    fn from(e: walkdir::Error) -> Self {
        Self::Walk(e)
    }
}

impl From<StripPrefixError> for LiterateError {
    fn from(e: StripPrefixError) -> Self {
        Self::Prefix(e)
    }
}

impl Display for LiterateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiterateError::IO(e) => e.fmt(f),
            LiterateError::Walk(e) => e.fmt(f),
            LiterateError::Prefix(e) => e.fmt(f),
            LiterateError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl Error for LiterateError {}
